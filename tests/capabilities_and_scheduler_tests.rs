use mlx_gen::capabilities::{CapabilitiesReport, CAPABILITIES_SCHEMA_VERSION};
use mlx_gen::config::quantization::{QuantizationMode, QuantizationPolicy};
use mlx_gen::pipeline::outpaint::PaddingSpec;
use mlx_gen::pipeline::scheduler::FlowMatchScheduler;

#[test]
fn test_capabilities_schema_version_and_contents() {
    let report = CapabilitiesReport::current();
    assert_eq!(report.schema_version, CAPABILITIES_SCHEMA_VERSION);
    assert_eq!(report.schema_version, 16);
    assert!(report.capabilities.len() >= 6);
    assert!(report.restoration.len() >= 2);

    let wan_entry = report.capabilities.iter().find(|c| c.model == "wan2.2-ti2v-5b");
    assert!(wan_entry.is_some());
    let wan = wan_entry.unwrap();
    assert!(wan.supports_video_mask);
    assert!(wan.supports_reference_images);
    assert_eq!(wan.default_flow_shift, Some(3.0));
}

#[test]
fn test_flow_match_scheduler_math() {
    let scheduler = FlowMatchScheduler::new(20, 3.0);
    assert_eq!(scheduler.timesteps.len(), 20);
    assert_eq!(scheduler.sigmas.len(), 21);

    // Initial sigma should be 1.0, terminal sigma should be 0.0
    assert!((scheduler.sigmas[0] - 1.0).abs() < 1e-4);
    assert!((scheduler.sigmas[20] - 0.0).abs() < 1e-4);

    let sample = vec![1.0f32, 2.0, 3.0];
    let model_out = vec![0.5f32, 0.5, 0.5];
    let stepped = scheduler.step_euler(0, &sample, &model_out);
    assert_eq!(stepped.len(), 3);
}

#[test]
fn test_padding_spec_math() {
    let pad = PaddingSpec::axis(100, 200);
    assert_eq!(pad.total_width(800), 1200);
    assert_eq!(pad.total_height(600), 800);
    assert!(pad.should_split_two_pass(400, 400));
}

#[test]
fn test_quantization_policy_preservation() {
    let policy = QuantizationPolicy::q8_balanced();
    assert_eq!(policy.mode, QuantizationMode::Q8);
    assert!(policy.should_preserve_tensor("transformer.blocks.0.norm1.weight"));
    assert!(policy.should_preserve_tensor("time_embed.weight"));
    assert!(!policy.should_preserve_tensor("transformer.blocks.0.attn.q_proj.weight"));
}
