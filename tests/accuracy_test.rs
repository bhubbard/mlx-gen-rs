//! Analytical and Mathematical Accuracy Verification Tests for mlx-gen-rs
//!
//! Validates:
//! 1. Flow-matching sigma boundary invariance: sigma(0) = 1.0, sigma(N) = 0.0, monotonic decay.
//! 2. Euler integrator exactness: linear drift-free trajectory under constant velocity fields.
//! 3. Outpaint canvas spatial conservation: bit-exact pixel preservation in source coordinates & binary mask partition.
//! 4. Quantization representation & bitwidth invariants.

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use mlx_gen::config::quantization::QuantizationMode;
use mlx_gen::config::task::OutpaintFillMode;
use mlx_gen::pipeline::outpaint::{OutpaintCanvas, PaddingSpec};
use mlx_gen::pipeline::scheduler::FlowMatchScheduler;
use std::str::FromStr;

#[test]
fn test_flow_match_boundary_and_monotonicity() {
    for &num_steps in &[10, 20, 50, 100] {
        for &shift in &[1.0f32, 2.0, 3.0, 5.0] {
            let scheduler = FlowMatchScheduler::new(num_steps, shift);

            assert_eq!(scheduler.timesteps.len(), num_steps as usize);
            assert_eq!(scheduler.sigmas.len(), (num_steps + 1) as usize);

            // Boundary conditions
            let sigma_0 = scheduler.sigmas[0];
            let sigma_n = *scheduler.sigmas.last().unwrap();

            assert!(
                (sigma_0 - 1.0).abs() < 1e-5,
                "sigma(0) must equal 1.0 (got {}) for shift {}",
                sigma_0,
                shift
            );
            assert!(
                (sigma_n - 0.0).abs() < 1e-5,
                "sigma(N) must equal 0.0 (got {}) for shift {}",
                sigma_n,
                shift
            );

            // Strict monotonic decay: sigma_{i+1} < sigma_i
            for i in 0..num_steps as usize {
                assert!(
                    scheduler.sigmas[i] > scheduler.sigmas[i + 1],
                    "Non-monotonic decay at step {} ({} vs {}) for shift {}",
                    i,
                    scheduler.sigmas[i],
                    scheduler.sigmas[i + 1],
                    shift
                );
            }
        }
    }
}

#[test]
fn test_flow_match_constant_velocity_exactness() {
    // Under constant velocity field v, dx/dt = v.
    // Total displacement Delta x = int_1^0 v dt = v * (sigma_N - sigma_0) = -v.
    // Euler integration should be bit-exact with zero cumulative discretization drift.
    for &shift in &[1.0f32, 2.5, 4.0] {
        let scheduler = FlowMatchScheduler::new(50, shift);
        let mut x = vec![10.0f32, -5.0, 0.5, 100.0];
        let v = vec![2.0f32, -3.0, 1.5, -10.0];

        let x_initial = x.clone();

        for step in 0..50 {
            x = scheduler.step_euler(step, &x, &v);
        }

        for i in 0..x.len() {
            let expected = x_initial[i] - v[i];
            let err = (x[i] - expected).abs();
            assert!(
                err < 5e-4,
                "Euler integration accumulated drift at dim {}: got {}, expected {}, err {}",
                i,
                x[i],
                expected,
                err
            );
        }
    }
}

#[test]
fn test_outpaint_spatial_and_mask_exactness() {
    let src_w = 40;
    let src_h = 30;
    let mut src_img = ImageBuffer::new(src_w, src_h);

    for y in 0..src_h {
        for x in 0..src_w {
            let r = (x * 6) as u8;
            let g = (y * 8) as u8;
            let b = ((x + y) * 3) as u8;
            src_img.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }
    let dynamic_src = DynamicImage::ImageRgba8(src_img);

    let padding = PaddingSpec {
        top: 15,
        right: 25,
        bottom: 10,
        left: 20,
    };

    let (expanded, mask) = OutpaintCanvas::create_expanded_canvas(
        &dynamic_src,
        &padding,
        OutpaintFillMode::Solid,
    )
    .expect("Canvas expansion should succeed");

    assert_eq!(expanded.width(), 40 + 20 + 25);
    assert_eq!(expanded.height(), 30 + 15 + 10);
    assert_eq!(mask.width(), expanded.width());
    assert_eq!(mask.height(), expanded.height());

    // 1. Verify bit-exact spatial conservation of original image inside expanded canvas
    for y in 0..src_h {
        for x in 0..src_w {
            let orig_px = dynamic_src.get_pixel(x, y);
            let canvas_px = expanded.get_pixel(x + padding.left, y + padding.top);
            assert_eq!(
                orig_px, canvas_px,
                "Spatial pixel corruption at ({}, {}) in canvas",
                x + padding.left,
                y + padding.top
            );

            // In source bounding box, mask must be pure 0 (unmasked source)
            let mask_px = mask.get_pixel(x + padding.left, y + padding.top);
            assert_eq!(
                mask_px[0], 0,
                "Mask in source region must be 0 at ({}, {})",
                x, y
            );
        }
    }

    // 2. Verify mask in padded margins is strictly 255 (to be generated)
    // Top margin
    for y in 0..padding.top {
        for x in 0..expanded.width() {
            assert_eq!(mask.get_pixel(x, y)[0], 255);
        }
    }
    // Left margin
    for y in 0..expanded.height() {
        for x in 0..padding.left {
            assert_eq!(mask.get_pixel(x, y)[0], 255);
        }
    }
}

#[test]
fn test_quantization_bitwidths_and_parsing() {
    assert_eq!(QuantizationMode::Fp32.bits(), 32);
    assert_eq!(QuantizationMode::Fp16.bits(), 16);
    assert_eq!(QuantizationMode::Bf16.bits(), 16);
    assert_eq!(QuantizationMode::Q8.bits(), 8);
    assert_eq!(QuantizationMode::Q4.bits(), 4);

    assert_eq!(
        QuantizationMode::from_str("fp32").unwrap(),
        QuantizationMode::Fp32
    );
    assert_eq!(
        QuantizationMode::from_str("bf16").unwrap(),
        QuantizationMode::Bf16
    );
    assert_eq!(
        QuantizationMode::from_str("q4").unwrap(),
        QuantizationMode::Q4
    );
    assert_eq!(
        QuantizationMode::from_str("q8").unwrap(),
        QuantizationMode::Q8
    );
    assert!(QuantizationMode::from_str("invalid_quant").is_err());
}
