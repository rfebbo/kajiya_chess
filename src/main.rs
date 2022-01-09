use anyhow::Context;
use dolly::prelude::*;

use imgui::im_str;
use kajiya::rg::GraphDebugHook;
use kajiya::world_renderer::{AddMeshOptions, InstanceHandle, WorldRenderer};
use kajiya_simple::*;

use std::fs::File;
use structopt::StructOpt;

// chess peices from https://www.thingiverse.com/thing:585218/files
// chess board from https://sketchfab.com/3d-models/chess-board-84ea7a7a4d794b78965c3a16e4aecf1d
const APP_STATE_CONFIG_FILE_PATH: &str = "view_state.ron";

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct SunState {
    theta: f32,
    phi: f32,
}

impl SunState {
    pub fn direction(&self) -> Vec3 {
        fn spherical_to_cartesian(theta: f32, phi: f32) -> Vec3 {
            let x = phi.sin() * theta.cos();
            let y = phi.cos();
            let z = phi.sin() * theta.sin();
            Vec3::new(x, y, z)
        }

        spherical_to_cartesian(self.theta, self.phi)
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct LocalLightsState {
    theta: f32,
    phi: f32,
    count: u32,
    distance: f32,
    multiplier: f32,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct PersistedAppState {
    camera_position: Vec3,
    camera_rotation: Quat,
    vertical_fov: f32,
    emissive_multiplier: f32,
    sun: SunState,
    lights: LocalLightsState,
    ev_shift: f32,
}

struct Game {
    render_instances: Vec<InstanceHandle>,
    locked_rg_debug_hook: Option<GraphDebugHook>,
    max_fps: u32,
    state: PersistedAppState,
    board_inst: InstanceHandle,
    camera: CameraRig,
    board_rotation: f32,
    board_distance: f32,
    keyboard: KeyboardState,
    keymap: KeyboardMap,
    mouse: MouseState,
}

impl Game {
    fn new(world_renderer: &mut WorldRenderer, aspect_ratio: f32) -> anyhow::Result<Game> {
        let camera = CameraRig::builder()
            .with(Position::new(Vec3::new(0.0, 8.5, 15.0)))
            .with(Rotation::new(Quat::from_rotation_x(-18.0f32.to_radians())))
            .with(LookAt::new(Vec3::ZERO))
            .build();

        // let car_mesh = world_renderer
        //     .add_baked_mesh("/baked/chess_board/scene.mesh", AddMeshOptions::new())?;
        let board_mesh = world_renderer
            .add_baked_mesh("/baked/chess_board/scene.mesh", AddMeshOptions::new())?;

        let bishop_mesh = world_renderer.add_baked_mesh(
            "/baked/chess_peices/chess_bishop.mesh",
            AddMeshOptions::new(),
        )?;

        let mut render_instances = vec![];

        let board_inst =
            world_renderer.add_instance(board_mesh, Vec3::ZERO, Quat::from_rotation_y(0.2));

        let bishop_inst =
            world_renderer.add_instance(bishop_mesh, Vec3::new(0.0, 2.1, 0.0), Quat::IDENTITY);

        render_instances.push(board_inst);
        render_instances.push(bishop_inst);

        let mouse: MouseState = Default::default();

        let persisted_app_state: Option<PersistedAppState> = File::open(APP_STATE_CONFIG_FILE_PATH)
            .ok()
            .and_then(|f| ron::de::from_reader(f).ok());

        let mut state = persisted_app_state
            .clone()
            .unwrap_or_else(|| PersistedAppState {
                camera_position: camera.final_transform.position,
                camera_rotation: camera.final_transform.rotation,
                emissive_multiplier: 1.0,
                vertical_fov: 52.0,
                sun: SunState {
                    theta: -4.54,
                    phi: 1.48,
                },
                lights: LocalLightsState {
                    theta: 1.0,
                    phi: 1.0,
                    count: 0,
                    distance: 1.5,
                    multiplier: 10.0,
                },
                ev_shift: 0.0,
            });

        const MAX_FPS_LIMIT: u32 = 256;
        let mut max_fps = MAX_FPS_LIMIT;

        let mut locked_rg_debug_hook: Option<GraphDebugHook> = None;

        // let state = &mut state;

        let keyboard: KeyboardState = Default::default();
        let keymap = KeyboardMap::new()
            .bind(
                VirtualKeyCode::W,
                KeyMap::new("up", 4.0).activation_time(1.0),
            )
            .bind(
                VirtualKeyCode::S,
                KeyMap::new("down", 4.0).activation_time(1.0),
            )
            .bind(
                VirtualKeyCode::A,
                KeyMap::new("left", 1.0).activation_time(1.0),
            )
            .bind(
                VirtualKeyCode::D,
                KeyMap::new("right", 1.0).activation_time(1.0),
            )
            .bind(
                VirtualKeyCode::LShift,
                KeyMap::new("nitro", 1.0).activation_time(0.0),
            );

        Ok(Game {
            render_instances,
            locked_rg_debug_hook,
            max_fps,
            state,
            board_inst,
            camera,
            board_rotation: 0.0,
            board_distance: 0.0,
            keyboard,
            keymap,
            mouse,
        })
    }

    fn frame(&mut self, mut ctx: FrameContext) -> WorldFrameDesc {
        // println!("{}", 1.0 / ctx.dt_filtered);

        self.keyboard.update(ctx.events);

        self.mouse.update(ctx.events);
        let input = self.keymap.map(&self.keyboard, ctx.dt_filtered);

        // ctx.world_renderer.set_instance_transform(
        //     self.board_inst,
        //     Vec3::ZERO,
        //     Quat::from_rotation_y(self.board_rotation),
        // );

        self.board_distance += input["up"] - input["down"];
        self.board_distance = self.board_distance.clamp(-5.0, 5.0);
        let cam_pos = Quat::from_rotation_y(self.board_rotation)
            * Vec3::new(1.0, self.board_distance + 8.0, 15.0);
        // cam_pos += self.camera.final_transform.right() * (-input["left"] + input["right"]);

        self.board_rotation += (-input["left"] + input["right"]) * 0.1;

        // let mut cam_rot = Quat::from_axis_angle(self.camera.final_transform.up(), self.cam_rot.x);
        // cam_rot *= Quat::from_axis_angle(self.camera.final_transform.right(), self.cam_rot.y);

        self.camera.driver_mut::<Position>().position = cam_pos;

        // self.camera.driver_mut::<Rotation>().rotation =
        //     Quat::from_axis_angle(Vec3::Y, self.board_rotation);

        let lens = CameraLens {
            aspect_ratio: ctx.aspect_ratio(),
            vertical_fov: self.state.vertical_fov,
            ..Default::default()
        };

        let camera_matrices = self
            .camera
            .update(ctx.dt_filtered)
            .into_position_rotation()
            .through(&lens);

        let mut debug_gi_cascade_idx: u32 = 0;

        for inst in &self.render_instances {
            ctx.world_renderer
                .get_instance_dynamic_parameters_mut(*inst)
                .emissive_multiplier = self.state.emissive_multiplier;
        }

        ctx.imgui.take().unwrap().frame(|ui| {
            if imgui::CollapsingHeader::new(im_str!("Tweaks"))
                .default_open(true)
                .build(ui)
            {
                imgui::Drag::<f32>::new(im_str!("EV shift"))
                    .range(-8.0..=8.0)
                    .speed(0.01)
                    .build(ui, &mut self.state.ev_shift);

                imgui::Drag::<f32>::new(im_str!("Emissive multiplier"))
                    .range(0.0..=10.0)
                    .speed(0.1)
                    .build(ui, &mut self.state.emissive_multiplier);

                imgui::Drag::<f32>::new(im_str!("Light intensity multiplier"))
                    .range(0.0..=1000.0)
                    .speed(1.0)
                    .build(ui, &mut self.state.lights.multiplier);

                imgui::Drag::<f32>::new(im_str!("Field of view"))
                    .range(1.0..=120.0)
                    .speed(0.25)
                    .build(ui, &mut self.state.vertical_fov);

                imgui::Drag::<f32>::new(im_str!("Sun size"))
                    .range(0.0..=10.0)
                    .speed(0.02)
                    .build(ui, &mut ctx.world_renderer.sun_size_multiplier);

                /*if ui.radio_button_bool(
                    im_str!("Move sun"),
                    left_click_edit_mode == LeftClickEditMode::MoveSun,
                ) {
                    left_click_edit_mode = LeftClickEditMode::MoveSun;
                }

                if ui.radio_button_bool(
                    im_str!("Move local lights"),
                    left_click_edit_mode == LeftClickEditMode::MoveLocalLights,
                ) {
                    left_click_edit_mode = LeftClickEditMode::MoveLocalLights;
                }

                imgui::Drag::<u32>::new(im_str!("Light count"))
                    .range(0..=10)
                    .build(ui, &mut state.lights.count);*/

                #[cfg(feature = "dlss")]
                {
                    ui.checkbox(im_str!("Use DLSS"), &mut ctx.world_renderer.use_dlss);
                }
            }

            /*if imgui::CollapsingHeader::new(im_str!("csgi"))
                .default_open(true)
                .build(ui)
            {
                imgui::Drag::<i32>::new(im_str!("Trace subdivision"))
                    .range(0..=5)
                    .build(ui, &mut world_renderer.csgi.trace_subdiv);

                imgui::Drag::<i32>::new(im_str!("Neighbors per frame"))
                    .range(1..=9)
                    .build(ui, &mut world_renderer.csgi.neighbors_per_frame);
            }*/

            if imgui::CollapsingHeader::new(im_str!("Debug"))
                .default_open(false)
                .build(ui)
            {
                if ui.radio_button_bool(
                    im_str!("Scene geometry"),
                    ctx.world_renderer.debug_mode == RenderDebugMode::None,
                ) {
                    ctx.world_renderer.debug_mode = RenderDebugMode::None;
                }

                if ui.radio_button_bool(
                    im_str!("GI voxel grid"),
                    matches!(
                        ctx.world_renderer.debug_mode,
                        RenderDebugMode::CsgiVoxelGrid { .. }
                    ),
                ) {
                    ctx.world_renderer.debug_mode = RenderDebugMode::CsgiVoxelGrid {
                        cascade_idx: debug_gi_cascade_idx as _,
                    };
                }

                if matches!(
                    ctx.world_renderer.debug_mode,
                    RenderDebugMode::CsgiVoxelGrid { .. }
                ) {
                    imgui::Drag::<u32>::new(im_str!("Cascade index"))
                        .range(0..=3)
                        .build(ui, &mut debug_gi_cascade_idx);

                    ctx.world_renderer.debug_mode = RenderDebugMode::CsgiVoxelGrid {
                        cascade_idx: debug_gi_cascade_idx as _,
                    };
                }

                if ui.radio_button_bool(
                    im_str!("GI voxel radiance"),
                    ctx.world_renderer.debug_mode == RenderDebugMode::CsgiRadiance,
                ) {
                    ctx.world_renderer.debug_mode = RenderDebugMode::CsgiRadiance;
                }

                imgui::ComboBox::new(im_str!("Shading")).build_simple_string(
                    ui,
                    &mut ctx.world_renderer.debug_shading_mode,
                    &[
                        im_str!("Default"),
                        im_str!("No base color"),
                        im_str!("Diffuse GI"),
                        im_str!("Reflections"),
                        im_str!("RTX OFF"),
                    ],
                );

                imgui::Drag::<u32>::new(im_str!("Max FPS"))
                    .range(1..=256)
                    .build(ui, &mut self.max_fps);
            }

            if imgui::CollapsingHeader::new(im_str!("GPU passes"))
                .default_open(true)
                .build(ui)
            {
                let gpu_stats = gpu_profiler::get_stats();
                ui.text(format!("CPU frame time: {:.3}ms", ctx.dt_filtered * 1000.0));

                let ordered_scopes = gpu_stats.get_ordered();
                let gpu_time_ms: f64 = ordered_scopes.iter().map(|(_, ms)| ms).sum();

                ui.text(format!("GPU frame time: {:.3}ms", gpu_time_ms));

                for (scope, ms) in ordered_scopes {
                    if scope.name == "debug" || scope.name.starts_with('_') {
                        continue;
                    }

                    let style = self.locked_rg_debug_hook.as_ref().and_then(|hook| {
                        if hook.render_scope == scope {
                            Some(ui.push_style_color(imgui::StyleColor::Text, [1.0, 1.0, 0.1, 1.0]))
                        } else {
                            None
                        }
                    });

                    ui.text(format!("{}: {:.3}ms", scope.name, ms));

                    if let Some(style) = style {
                        style.pop(ui);
                    }

                    if ui.is_item_hovered() {
                        ctx.world_renderer.rg_debug_hook = Some(kajiya::rg::GraphDebugHook {
                            render_scope: scope.clone(),
                        });

                        if ui.is_item_clicked(imgui::MouseButton::Left) {
                            if self.locked_rg_debug_hook == ctx.world_renderer.rg_debug_hook {
                                self.locked_rg_debug_hook = None;
                            } else {
                                self.locked_rg_debug_hook =
                                    ctx.world_renderer.rg_debug_hook.clone();
                            }
                        }
                    }
                }
            }
        });

        ctx.world_renderer.ev_shift = self.state.ev_shift;

        WorldFrameDesc {
            camera_matrices,
            render_extent: ctx.render_extent,
            sun_direction: Vec3::new(4.0, 1.0, 1.0).normalize(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Point `kajiya` to standard assets and shaders in the parent directory
    set_standard_vfs_mount_points("../kajiya");

    // Game-specific assets in the current directory
    set_vfs_mount_point("/baked", "./baked");

    let mut kajiya = SimpleMainLoop::builder().resolution([2560, 1440]).build(
        WindowBuilder::new()
            .with_title("hello-kajiya")
            .with_resizable(false),
    )?;

    let aspect_ratio = kajiya.window_aspect_ratio();

    let mut game = Game::new(&mut kajiya.world_renderer, aspect_ratio)?;

    kajiya.run(move |ctx| game.frame(ctx))
}
