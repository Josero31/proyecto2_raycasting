use raylib::prelude::*;
use rand::Rng;

struct Tree {
    x: f32,
    z: f32,
    height: f32,
    leaf_layers: i32,
    tree_type: TreeType,
}

#[derive(Clone, Copy)]
enum TreeType {
    Oak,
    Cherry,
    Birch,
}

struct Rock {
    x: f32,
    z: f32,
    size: f32,
}

struct Flower {
    x: f32,
    z: f32,
    color: Color,
}

fn is_in_shadow(x: f32, y: f32, z: f32, trees: &Vec<Tree>, rocks: &Vec<Rock>, light_dir: Vector3) -> bool {
    let shadow_steps = 15;
    let step_size = 0.6;
    
    for i in 1..shadow_steps {
        let test_pos = Vector3::new(
            x - light_dir.x * i as f32 * step_size,
            y - light_dir.y * i as f32 * step_size,
            z - light_dir.z * i as f32 * step_size,
        );
        
        for tree in trees {
            let dx = test_pos.x - tree.x;
            let dz = test_pos.z - tree.z;
            let dist = (dx * dx + dz * dz).sqrt();
            
            if dist < 0.5 && test_pos.y >= 0.0 && test_pos.y <= tree.height {
                return true;
            }
            
            for layer in 0..tree.leaf_layers {
                let layer_y = tree.height + layer as f32 * 0.8;
                let size = (tree.leaf_layers - layer) as f32 * 0.8 + 1.5;
                if dist < size / 2.0 && (test_pos.y - layer_y).abs() < 0.6 {
                    return true;
                }
            }
        }
        
        for rock in rocks {
            let dx = test_pos.x - rock.x;
            let dz = test_pos.z - rock.z;
            let dist = (dx * dx + dz * dz).sqrt();
            
            let rock_height = ((rock.x * 0.3).sin() + (rock.z * 0.3).cos()) * 2.5
                + ((rock.x * 0.5).sin() * (rock.z * 0.5).cos()) * 1.5
                + 2.0;
            
            if dist < rock.size / 2.0 && (test_pos.y - rock_height).abs() < rock.size {
                return true;
            }
        }
        
        if test_pos.y > 60.0 {
            break;
        }
    }
    
    false
}

fn apply_shadow(base_color: Color, in_shadow: bool) -> Color {
    if in_shadow {
        Color::new(
            (base_color.r as f32 * 0.5) as u8,
            (base_color.g as f32 * 0.5) as u8,
            (base_color.b as f32 * 0.5) as u8,
            base_color.a,
        )
    } else {
        base_color
    }
}

fn main() {
    println!("Iniciando Diorama Minecraft...");
    
    let (mut rl, thread) = raylib::init()
        .size(1400, 900)
        .title("Diorama Minecraft - Paisaje al Atardecer")
        .build();
    
    rl.set_target_fps(60);
    
    let mut camera = Camera3D::perspective(
        Vector3::new(30.0, 30.0, 50.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );
    
    let mut rng = rand::thread_rng();
    let mut trees: Vec<Tree> = Vec::new();
    
    for _ in 0..35 {
        let tree_type = match rng.gen_range(0..3) {
            0 => TreeType::Oak,
            1 => TreeType::Cherry,
            _ => TreeType::Birch,
        };
        
        trees.push(Tree {
            x: rng.gen_range(-25.0..25.0),
            z: rng.gen_range(-25.0..25.0),
            height: rng.gen_range(8.0..15.0),
            leaf_layers: rng.gen_range(3..6),
            tree_type,
        });
    }
    
    let mut rocks: Vec<Rock> = Vec::new();
    for _ in 0..20 {
        rocks.push(Rock {
            x: rng.gen_range(-25.0..25.0),
            z: rng.gen_range(-25.0..25.0),
            size: rng.gen_range(2.0..5.0),
        });
    }
    
    let mut flowers: Vec<Flower> = Vec::new();
    for _ in 0..50 {
        flowers.push(Flower {
            x: rng.gen_range(-25.0..25.0),
            z: rng.gen_range(-25.0..25.0),
            color: if rng.gen_bool(0.5) {
                Color::new(255, 50, 50, 255)
            } else {
                Color::new(255, 255, 50, 255)
            },
        });
    }
    
    let light_dir = Vector3::new(-0.5, -0.7, -0.3).normalized();
    
    let mut rotation_angle: f32 = 0.0;
    let mut zoom: f32 = 50.0;
    let mut is_dragging = false;
    let mut last_mouse_pos = Vector2::zero();
    
    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            is_dragging = true;
            last_mouse_pos = rl.get_mouse_position();
        }
        
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            is_dragging = false;
        }
        
        if is_dragging {
            let current_mouse = rl.get_mouse_position();
            let delta = current_mouse.x - last_mouse_pos.x;
            rotation_angle += delta * 0.005;
            last_mouse_pos = current_mouse;
        }
        
        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0.0 {
            zoom -= wheel * 5.0;
            zoom = zoom.clamp(20.0, 100.0);
        }
        
        camera.position = Vector3::new(
            rotation_angle.sin() * zoom,
            30.0,
            rotation_angle.cos() * zoom,
        );
        camera.target = Vector3::new(0.0, 0.0, 0.0);
        
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(135, 206, 235, 255));
        
        let mut d3 = d.begin_mode3D(camera);
        
        for x in -25..25 {
            for z in -25..25 {
                let fx = x as f32;
                let fz = z as f32;
                let height = ((fx * 0.1).sin() + (fz * 0.1).cos()) * 1.5;
                
                let in_shadow = is_in_shadow(fx, height, fz, &trees, &rocks, light_dir);
                
                let river_center = 0.0;
                let river_width = 4.0;
                let distance_to_river = (fz - river_center).abs();
                
                if distance_to_river < river_width {
                    let water_color = Color::new(30, 144, 255, 180);
                    let water_y = -0.5;
                    d3.draw_cube(
                        Vector3::new(fx, water_y, fz),
                        2.0, 0.3, 2.0,
                        apply_shadow(water_color, in_shadow)
                    );
                } else {
                    let grass_color = if distance_to_river < river_width + 2.0 {
                        Color::new(34, 139, 34, 255)
                    } else {
                        Color::new(50, 205, 50, 255)
                    };
                    
                    d3.draw_cube(
                        Vector3::new(fx, height - 1.0, fz),
                        2.0, 2.0, 2.0,
                        apply_shadow(grass_color, in_shadow)
                    );
                }
            }
        }
        
        for tree in &trees {
            let trunk_color = match tree.tree_type {
                TreeType::Oak => Color::new(139, 69, 19, 255),
                TreeType::Cherry => Color::new(160, 82, 45, 255),
                TreeType::Birch => Color::new(245, 245, 220, 255),
            };
            
            let trunk_in_shadow = is_in_shadow(tree.x, tree.height / 2.0, tree.z, &trees, &rocks, light_dir);
            d3.draw_cube(
                Vector3::new(tree.x, tree.height / 2.0, tree.z),
                1.0, tree.height, 1.0,
                apply_shadow(trunk_color, trunk_in_shadow)
            );
            
            for layer in 0..tree.leaf_layers {
                let layer_y = tree.height + layer as f32 * 0.8;
                let size = (tree.leaf_layers - layer) as f32 * 0.8 + 1.5;
                
                let leaf_color = match tree.tree_type {
                    TreeType::Oak => Color::new(34, 139, 34, 255),
                    TreeType::Cherry => Color::new(255, 182, 193, 255),
                    TreeType::Birch => Color::new(144, 238, 144, 255),
                };
                
                let leaf_in_shadow = is_in_shadow(tree.x, layer_y, tree.z, &trees, &rocks, light_dir);
                d3.draw_cube(
                    Vector3::new(tree.x, layer_y, tree.z),
                    size, size * 0.8, size,
                    apply_shadow(leaf_color, leaf_in_shadow)
                );
            }
        }
        
        for rock in &rocks {
            let rock_height = ((rock.x * 0.3).sin() + (rock.z * 0.3).cos()) * 2.5
                + ((rock.x * 0.5).sin() * (rock.z * 0.5).cos()) * 1.5
                + 2.0;
            
            let rock_in_shadow = is_in_shadow(rock.x, rock_height, rock.z, &trees, &rocks, light_dir);
            d3.draw_cube(
                Vector3::new(rock.x, rock_height, rock.z),
                rock.size, rock.size * 1.2, rock.size,
                apply_shadow(Color::new(128, 128, 128, 255), rock_in_shadow)
            );
        }
        
        for flower in &flowers {
            let flower_y = ((flower.x * 0.1).sin() + (flower.z * 0.1).cos()) * 1.5 + 0.5;
            let flower_in_shadow = is_in_shadow(flower.x, flower_y, flower.z, &trees, &rocks, light_dir);
            
            d3.draw_cube(
                Vector3::new(flower.x, flower_y, flower.z),
                0.3, 1.0, 0.3,
                apply_shadow(Color::new(34, 139, 34, 255), flower_in_shadow)
            );
            
            d3.draw_cube(
                Vector3::new(flower.x, flower_y + 0.5, flower.z),
                0.5, 0.5, 0.5,
                apply_shadow(flower.color, flower_in_shadow)
            );
        }
    }
}
