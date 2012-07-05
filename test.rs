use glut;   // FIXME: Should only be for tests.
import glut::{init};
import glut::bindgen::{glutInitDisplayMode, glutMainLoop, glutSwapBuffers};
import opengles::gl2::{ARRAY_BUFFER, COLOR_BUFFER_BIT, COMPILE_STATUS};
import opengles::gl2::{FRAGMENT_SHADER, LINK_STATUS, NO_ERROR, STATIC_DRAW};
import opengles::gl2::{TRIANGLE_STRIP, VERTEX_SHADER, GLclampf, GLenum};
import opengles::gl2::{GLsizei, GLuint, attach_shader, bind_buffer};
import opengles::gl2::{buffer_data, create_program, clear, clear_color};
import opengles::gl2::{compile_shader, create_shader, draw_arrays};
import opengles::gl2::{enable_vertex_attrib_array, gen_buffers};
import opengles::gl2::{get_attrib_location, get_error, get_program_iv};
import opengles::gl2::{get_shader_info_log, get_shader_iv};
import opengles::gl2::{get_uniform_location, link_program, shader_source};
import opengles::gl2::{use_program, vertex_attrib_pointer_f32};

import comm::{chan, port, recv, send};
import io::println;
import ptr::{addr_of, null};
import str::bytes;
import task::{builder, get_opts, run_listener, set_opts};
import vec::unsafe::to_ptr;

fn fragment_shader_source() -> str {
    "
    #ifdef GLES2
        precision mediump float;
    #endif

        void main(void) {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "
}

fn vertex_shader_source() -> str {
    "
        attribute vec3 aVertexPosition;

        /*uniform mat4 uMVMatrix;
        uniform mat4 uPMatrix;*/

        void main(void) {
            gl_Position = /*uPMatrix * uMVMatrix **/
                vec4(aVertexPosition, 1.0);
        }
    "
}

fn load_shader(source_str: str, shader_type: GLenum) -> GLuint {
    let shader_id = create_shader(shader_type);
    shader_source(shader_id, [bytes(source_str)]);
    compile_shader(shader_id);

    if get_error() != NO_ERROR {
        println(#fmt("error: %d", get_error() as int));
        fail "failed to compile shader with error";
    }

    if get_shader_iv(shader_id, COMPILE_STATUS) == (0 as GLint) {
        println(get_shader_info_log(shader_id));
        fail "failed to compile shader";
    }
    ret shader_id;
}

class shader_program {
    let program: GLuint;
    let aVertexPosition: c_int;
    /*let uPMatrix: c_int;
    let uMVMatrix: c_int;*/

    new(program: GLuint) {
        self.program = program;
        self.aVertexPosition = get_attrib_location(program, "aVertexPosition");
        /*self.uPMatrix = get_uniform_location(program, "uPMatrix");
        self.uMVMatrix = get_uniform_location(program, "uMVMatrix");*/

        enable_vertex_attrib_array(self.aVertexPosition as GLuint);
    }
}

fn init_shaders() -> shader_program {
    let vertex_shader = load_shader(vertex_shader_source(), VERTEX_SHADER);
    let fragment_shader = load_shader(fragment_shader_source(),
                                      FRAGMENT_SHADER);

    let program = create_program();
    attach_shader(program, vertex_shader);
    attach_shader(program, fragment_shader);
    link_program(program);

    if get_program_iv(program, LINK_STATUS) == (0 as GLint) {
        fail "failed to initialize program";
    }

    use_program(program);

    ret shader_program(program);
}

fn init_buffers() -> GLuint {
    let triangle_vertex_buffer = gen_buffers(1 as GLsizei)[0];
    bind_buffer(ARRAY_BUFFER, triangle_vertex_buffer);
    let vertices = [
        0.0f32, 1.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        0.0f32, 0.0f32, 0.0f32
    ];
    buffer_data(ARRAY_BUFFER, vertices, STATIC_DRAW);
    ret triangle_vertex_buffer;
}

fn draw_scene(shader_program: shader_program, vertex_buffer: GLuint) {
    clear_color(0.0f32, 0.0f32, 1.0f32, 1.0f32);
    clear(COLOR_BUFFER_BIT);

    bind_buffer(ARRAY_BUFFER, vertex_buffer);
    vertex_attrib_pointer_f32(shader_program.aVertexPosition as GLuint,
                              3 as GLint, false, 0 as GLsizei, 0 as GLuint);
    draw_arrays(TRIANGLE_STRIP, 0 as GLint, 3 as GLint);
}

extern fn display_callback() {
    let program = init_shaders();
    let vertex_buffer = init_buffers();
    draw_scene(program, vertex_buffer);

    glutSwapBuffers();
}

#[test]
fn test_triangle_and_square() unsafe {
    let builder = builder();
    let opts = {
        sched: some({ mode: task::osmain, native_stack_size: none })
        with get_opts(builder)
    };
    set_opts(builder, opts);

    let port: port<()> = port();
    let chan = chan(port);
    let _result_ch: chan<()> = run_listener(builder, {
        |_port|

        init();
        glutInitDisplayMode(0 as c_uint);
        create_window("Rust GLUT");
        display_func(display_callback);
        glutMainLoop();

        send(chan, ());
    });
    recv(port);
}

