// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern mod opengles;   // FIXME: Should only be for tests.
use glut::{swap_buffers, GLint};
use self::opengles::gl2::{ARRAY_BUFFER, COLOR_BUFFER_BIT, COMPILE_STATUS};
use self::opengles::gl2::{FRAGMENT_SHADER, LINK_STATUS, NO_ERROR, STATIC_DRAW};
use self::opengles::gl2::{TRIANGLE_STRIP, VERTEX_SHADER, GLenum};
use self::opengles::gl2::{GLsizei, GLuint, attach_shader, bind_buffer};
use self::opengles::gl2::{buffer_data, create_program, clear, clear_color};
use self::opengles::gl2::{compile_shader, create_shader, draw_arrays};
use self::opengles::gl2::{enable_vertex_attrib_array, gen_buffers};
use self::opengles::gl2::{get_attrib_location, get_error, get_program_iv};
use self::opengles::gl2::{get_shader_info_log, get_shader_iv};
use self::opengles::gl2::{link_program, shader_source};
use self::opengles::gl2::{use_program, vertex_attrib_pointer_f32};

use std::comm;
use std::libc::c_int;
use std::io::println;
use std::task;

fn fragment_shader_source() -> ~str {
    ~"
    #ifdef GLES2
        precision mediump float;
    #endif

        void main(void) {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "
}

fn vertex_shader_source() -> ~str {
    ~"
        attribute vec3 aVertexPosition;

        /*uniform mat4 uMVMatrix;
        uniform mat4 uPMatrix;*/

        void main(void) {
            gl_Position = /*uPMatrix * uMVMatrix **/
                vec4(aVertexPosition, 1.0);
        }
    "
}

fn load_shader(source_str: ~str, shader_type: GLenum) -> GLuint {
    let shader_id = create_shader(shader_type);
    shader_source(shader_id, [source_str.as_bytes().to_owned()]);
    compile_shader(shader_id);

    if get_error() != NO_ERROR {
        println(fmt!("error: %d", get_error() as int));
        fail!(~"failed to compile shader with error");
    }

    if get_shader_iv(shader_id, COMPILE_STATUS) == (0 as GLint) {
        println(get_shader_info_log(shader_id));
        fail!(~"failed to compile shader");
    }
    return shader_id;
}

struct ShaderProgram {
    program: GLuint,
    aVertexPosition: c_int,
    /*let uPMatrix: c_int;
    let uMVMatrix: c_int;*/
}

fn ShaderProgram(program: GLuint) -> ShaderProgram {
    let p = ShaderProgram {
        program : program,
        aVertexPosition : get_attrib_location(program, ~"aVertexPosition"),
        /*self.uPMatrix : get_uniform_location(program, "uPMatrix"),
        self.uMVMatrix : get_uniform_location(program, "uMVMatrix")*/
    };
    enable_vertex_attrib_array(p.aVertexPosition as GLuint);
    return p;
}

fn init_shaders() -> ShaderProgram {
    let vertex_shader = load_shader(vertex_shader_source(), VERTEX_SHADER);
    let fragment_shader = load_shader(fragment_shader_source(),
                                      FRAGMENT_SHADER);

    let program = create_program();
    attach_shader(program, vertex_shader);
    attach_shader(program, fragment_shader);
    link_program(program);

    if get_program_iv(program, LINK_STATUS) == (0 as GLint) {
        fail!(~"failed to initialize program");
    }

    use_program(program);

    return ShaderProgram(program);
}

fn init_buffers() -> GLuint {
    let triangle_vertex_buffer = gen_buffers(1 as GLsizei)[0];
    bind_buffer(ARRAY_BUFFER, triangle_vertex_buffer);
    let vertices = ~[
        0.0f32, 1.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        0.0f32, 0.0f32, 0.0f32
    ];
    buffer_data(ARRAY_BUFFER, vertices, STATIC_DRAW);
    return triangle_vertex_buffer;
}

fn draw_scene(shader_program: ShaderProgram, vertex_buffer: GLuint) {
    clear_color(0.0f32, 0.0f32, 1.0f32, 1.0f32);
    clear(COLOR_BUFFER_BIT);

    bind_buffer(ARRAY_BUFFER, vertex_buffer);
    vertex_attrib_pointer_f32(shader_program.aVertexPosition as GLuint,
                              3 as GLint, false, 0 as GLsizei, 0 as GLuint);
    draw_arrays(TRIANGLE_STRIP, 0 as GLint, 3 as GLint);
}

fn display_callback() {
    let program = init_shaders();
    let vertex_buffer = init_buffers();
    draw_scene(program, vertex_buffer);

    swap_buffers();
}

/*
#[test]
fn test_triangle_and_square() {
    let (po, ch): (Port<()>, Chan<()>) = stream();
    let _result_ch: Chan<()> = do spawn_listener | _po | {
        init();
        init_display_mode(DOUBLE);
        let window = create_window(~"Rust GLUT");
        display_func(display_callback);

        let (wakeup, wakeup_chan): (Port<()>, Chan<()>) = stream();

        do timer_func(1000) { wakeup_chan.send(()); }

        loop {
            check_loop();

            if wakeup.peek() {
                wakeup.recv();
                ch.send(());
                destroy_window(window);
                break;
            }
        }
    };

    po.recv();
}
*/

pub fn spawn_listener<A: Send>(f: ~fn(Port<A>)) -> Chan<A> {
    let (setup_po, setup_ch) = comm::stream();
    do task::spawn {
        let (po, ch) = comm::stream();
        setup_ch.send(ch);
        f(po);
    }
    setup_po.recv()
}
