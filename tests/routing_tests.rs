use mlx_gen::config::task::TaskType;
use mlx_gen::pipeline::router::{GenerationRequest, TaskRouter};
use std::path::PathBuf;

#[test]
fn test_wan_text_to_video_inference() {
    let req = GenerationRequest {
        model: "wan2.2-ti2v-5b".to_string(),
        task: TaskType::Auto,
        prompt: "A soaring eagle over snow-capped mountains".to_string(),
        negative_prompt: None,
        images: vec![],
        videos: vec![],
        reference_images: vec![],
        mask: None,
        video_mask: None,
        width: None,
        height: None,
        frames: None,
        fps: None,
        steps: None,
        guidance: None,
        seed: Some(42),
        flow_shift: None,
        output_path: PathBuf::from("output.mp4"),
        low_ram: false,
        json_events: false,
    };

    let route = TaskRouter::resolve(&req).expect("Failed to resolve route");
    assert_eq!(route.task, TaskType::TextToVideo);
    assert_eq!(route.target_width, 832);
    assert_eq!(route.target_height, 480);
    assert_eq!(route.target_frames, Some(81));
    assert_eq!(route.target_fps, Some(16.0));
    assert_eq!(route.flow_shift, Some(3.0));
}

#[test]
fn test_wan_image_to_video_inference() {
    let req = GenerationRequest {
        model: "wan2.2-ti2v-5b".to_string(),
        task: TaskType::Auto,
        prompt: "Animate waves moving".to_string(),
        negative_prompt: None,
        images: vec![PathBuf::from("beach.png")],
        videos: vec![],
        reference_images: vec![],
        mask: None,
        video_mask: None,
        width: Some(1280),
        height: Some(720),
        frames: Some(49),
        fps: Some(24.0),
        steps: Some(30),
        guidance: Some(5.0),
        seed: None,
        flow_shift: None,
        output_path: PathBuf::from("beach.mp4"),
        low_ram: false,
        json_events: false,
    };

    let route = TaskRouter::resolve(&req).expect("Failed to resolve route");
    assert_eq!(route.task, TaskType::ImageToVideo);
    assert_eq!(route.target_width, 1280);
    assert_eq!(route.target_height, 720);
    assert_eq!(route.target_frames, Some(49));
    assert_eq!(route.target_fps, Some(24.0));
    assert_eq!(route.target_steps, 30);
}

#[test]
fn test_flux_schnell_routing() {
    let req = GenerationRequest {
        model: "flux-schnell".to_string(),
        task: TaskType::Auto,
        prompt: "Futuristic city skyline at sunset".to_string(),
        negative_prompt: None,
        images: vec![],
        videos: vec![],
        reference_images: vec![],
        mask: None,
        video_mask: None,
        width: None,
        height: None,
        frames: None,
        fps: None,
        steps: None,
        guidance: None,
        seed: None,
        flow_shift: None,
        output_path: PathBuf::from("city.png"),
        low_ram: false,
        json_events: false,
    };

    let route = TaskRouter::resolve(&req).expect("Failed to resolve route");
    assert_eq!(route.task, TaskType::TextToImage);
    assert_eq!(route.target_steps, 4);
    assert_eq!(route.target_width, 1024);
    assert_eq!(route.target_height, 1024);
}
