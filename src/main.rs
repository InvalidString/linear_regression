use raylib::prelude::*;
mod mat;

fn plot(g: &mut impl RaylibDraw, f: impl Fn(f32)->f32, color: Color){
    let mut last_pos = Vector2{x: -1000.0, y: 0.0};
    for x in -1000..1000{
        let x = x as f32;
        let y = f(x);
        let pos = Vector2 {x, y};
        g.draw_line_v(last_pos, pos, color);
        last_pos = pos;
    }
}


fn main() {
    let (mut rl, t) = init()
        .vsync()
        .title("Demo")
        .resizable()
        .size(800, 600)
        .build();

    let mut cam = Camera2D::default();
    cam.target = Vector2::zero();
    cam.zoom = 5.0;


    let mut points = vec![];

    let functions = [
        |_|1.0,
        |x|x,
        |x|x*x,
        |x|x*x*x,
        |x|x*x*x*x,
    ];


    let _functions = [
        |_:f32|1.0,
        //|x|f32::sin(x),
        //|x|f32::sin(x/2.0),
        //|x|f32::sin(x/3.0),
        //|x|f32::sin(x/4.0),
        |x|f32::sin(x/5.0),
        |x|f32::sin(x/6.0),
        |x|f32::sin(x/7.0),
        |x|f32::sin(x/8.0),
    ];

    let mut last_sol = None;

    while !rl.window_should_close() {
        cam.offset = Vector2{
            x: rl.get_screen_width() as f32,
            y: rl.get_screen_height() as f32,
        } * 0.5;

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON){
            let m_pos = Vector2{
                x: rl.get_mouse_x() as f32,
                y: rl.get_mouse_y() as f32,
            };
            let m_world_pos = rl.get_screen_to_world2D(m_pos, cam);

            points.push(m_world_pos);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_C){
            points.clear();
        }


        //Drawing
        let mut g = rl.begin_drawing(&t);
        g.clear_background(Color::BLACK);
        g.draw_fps(10, 10);

        let solution = if points.len() > 10{
            #[allow(non_snake_case)]
            let A = mat::Matrix::by_pos(points.len(), functions.len(), |y,x|{
                functions[x](points[y].x)
            });
            let v = mat::Matrix::by_pos(points.len(), 1, |y,_|points[y].y);

            #[allow(non_snake_case)]
            let At = A.transpose();

            let mut lhs = At.clone() * &A;
            let mut rhs = At * &v;

            lhs.solve(&mut rhs);

            g.draw_text(&format!("{}", rhs), 100, 10, 1, Color::YELLOW);
            g.draw_text(&format!("{}", lhs), 200, 10, 1, Color::YELLOW);

            Some(rhs)
        }else{None};

        if solution != last_sol{
            if let Some(sol) = &solution{
                println!("{:#?}", sol.data_slice());
            }
            last_sol = solution.clone();
        }

        //2d
        {
            let mut g = g.begin_mode2D(cam);

            for p in &points{
                g.draw_circle_v(p, 1.0, Color::BLUE);
            }


            if let Some(solution) = solution{
                plot(&mut g, |x|{
                    functions.into_iter().zip(solution.rows())
                        .map(|(f,mag)| f(x)*mag[0])
                        .sum()
                }, Color::GREEN);
            }

            //plot(&mut g, |x|f32::sin(x), Color::GRAY);

            g.draw_line(1, 1, -1, -1, Color::WHITE);
            g.draw_line(1, -1, -1, 1, Color::WHITE);
        }

    }
}
