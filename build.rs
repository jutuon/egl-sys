extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, StaticGenerator, GlobalGenerator, Generator};
use std::env;
use std::fs::File;
use std::path::{Path};
use std::io::{Write};

const FILE_BEGINNING_TEXT: &'static [u8] = b"//
//
// This file is automatically generated.
//
//
";

#[cfg(feature = "EGL_version_1_4")]
const EGL_VERSION: (u8, u8) = (1, 4);

#[cfg(feature = "EGL_version_1_5")]
const EGL_VERSION: (u8, u8) = (1, 5);


const LATEST_EGL_VERSION: (u8, u8) = (1, 5);


macro_rules! extensions {
    ( $( $x: tt )* )  => {
        {
            let extension_strings: &[&'static str]  = &[
                $(
                    #[cfg(feature = $x)] $x,
                )*
            ];

            extension_strings
        }
    };
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let file_path = Path::new(&out_dir).join("egl_generated_bindings.rs");

    #[cfg(feature = "function_pointer_loading")]
    generate_bindings(file_path, EGL_VERSION, GlobalGenerator, &[]);

    #[cfg(not(feature = "function_pointer_loading"))]
    generate_bindings(file_path, EGL_VERSION, StaticGenerator, &[]);


    // Following extension list is from
    // https://www.khronos.org/registry/EGL/
    let extension_strings = extensions!(
        "EGL_KHR_config_attribs"
        "EGL_KHR_lock_surface"
        "EGL_KHR_image"
        "EGL_KHR_vg_parent_image"
        "EGL_KHR_gl_texture_2D_image"
        "EGL_KHR_gl_texture_cubemap_image"
        "EGL_KHR_gl_texture_3D_image"
        "EGL_KHR_gl_renderbuffer_image"
        "EGL_KHR_reusable_sync"
        "EGL_KHR_image_base"
        "EGL_KHR_image_pixmap"
        "EGL_IMG_context_priority"
        "EGL_NOK_texture_from_pixmap"
        "EGL_KHR_lock_surface2"
        "EGL_NV_coverage_sample"
        "EGL_NV_depth_nonlinear"
        "EGL_NV_sync"
        "EGL_KHR_fence_sync"
        "EGL_NOK_swap_region2"
        "EGL_HI_clientpixmap"
        "EGL_HI_colorformats"
        "EGL_MESA_drm_image"
        "EGL_NV_post_sub_buffer"
        "EGL_ANGLE_query_surface_pointer"
        "EGL_ANGLE_surface_d3d_texture_2d_share_handle"
        "EGL_NV_coverage_sample_resolve"
        "EGL_NV_system_time"
        "EGL_KHR_stream"
        "EGL_KHR_stream_attrib"
        "EGL_KHR_stream_consumer_gltexture"
        "EGL_KHR_stream_producer_eglsurface"
        "EGL_KHR_stream_producer_aldatalocator"
        "EGL_KHR_stream_fifo"
        "EGL_EXT_create_context_robustness"
        "EGL_ANGLE_d3d_share_handle_client_buffer"
        "EGL_KHR_create_context"
        "EGL_KHR_surfaceless_context"
        "EGL_KHR_stream_cross_process_fd"
        "EGL_EXT_multiview_window"
        "EGL_KHR_wait_sync"
        "EGL_NV_post_convert_rounding"
        "EGL_NV_native_query"
        "EGL_NV_3dvision_surface"
        "EGL_ANDROID_framebuffer_target"
        "EGL_ANDROID_blob_cache"
        "EGL_ANDROID_image_native_buffer"
        "EGL_ANDROID_native_fence_sync"
        "EGL_ANDROID_recordable"
        "EGL_EXT_buffer_age"
        "EGL_EXT_image_dma_buf_import"
        "EGL_ARM_pixmap_multisample_discard"
        "EGL_EXT_swap_buffers_with_damage"
        "EGL_NV_stream_sync"
        "EGL_EXT_platform_base"
        "EGL_EXT_client_extensions"
        "EGL_EXT_platform_x11"
        "EGL_KHR_cl_event"
        "EGL_KHR_get_all_proc_addresses"
        "EGL_KHR_client_get_all_proc_addresses"
        "EGL_MESA_platform_gbm"
        "EGL_EXT_platform_wayland"
        "EGL_KHR_lock_surface3"
        "EGL_KHR_cl_event2"
        "EGL_KHR_gl_colorspace"
        "EGL_EXT_protected_surface"
        "EGL_KHR_platform_android"
        "EGL_KHR_platform_gbm"
        "EGL_KHR_platform_wayland"
        "EGL_KHR_platform_x11"
        "EGL_EXT_device_base"
        "EGL_EXT_platform_device"
        "EGL_NV_device_cuda"
        "EGL_NV_cuda_event"
        "EGL_TIZEN_image_native_buffer"
        "EGL_TIZEN_image_native_surface"
        "EGL_EXT_output_base"
        "EGL_EXT_device_drm"
        "EGL_EXT_output_drm"
        "EGL_EXT_device_openwf"
        "EGL_EXT_output_openwf"
        "EGL_EXT_stream_consumer_egloutput"
        "EGL_KHR_partial_update"
        "EGL_KHR_swap_buffers_with_damage"
        "EGL_ANGLE_window_fixed_size"
        "EGL_EXT_yuv_surface"
        "EGL_MESA_image_dma_buf_export"
        "EGL_EXT_device_enumeration"
        "EGL_EXT_device_query"
        "EGL_ANGLE_device_d3d"
        "EGL_KHR_create_context_no_error"
        "EGL_KHR_debug"
        "EGL_NV_stream_metadata"
        "EGL_NV_stream_consumer_gltexture_yuv"
        "EGL_IMG_image_plane_attribs"
        "EGL_KHR_mutable_render_buffer"
        "EGL_EXT_protected_content"
        "EGL_ANDROID_presentation_time"
        "EGL_ANDROID_create_native_client_buffer"
        "EGL_ANDROID_front_buffer_auto_refresh"
        "EGL_KHR_no_config_context"
        "EGL_KHR_context_flush_control"
        "EGL_ARM_implicit_external_sync"
        "EGL_MESA_platform_surfaceless"
        "EGL_EXT_image_dma_buf_import_modifiers"
        "EGL_EXT_pixel_format_float"
        "EGL_EXT_gl_colorspace_bt2020_linear"
        "EGL_EXT_gl_colorspace_bt2020_pq"
        "EGL_EXT_gl_colorspace_scrgb_linear"
        "EGL_EXT_surface_SMPTE2086_metadata"
        "EGL_NV_stream_fifo_next"
        "EGL_NV_stream_fifo_synchronous"
        "EGL_NV_stream_reset"
        "EGL_NV_stream_frame_limits"
        "EGL_NV_stream_remote"
        "EGL_NV_stream_cross_object"
        "EGL_NV_stream_cross_display"
        "EGL_NV_stream_cross_process"
        "EGL_NV_stream_cross_partition"
        "EGL_NV_stream_cross_system"
        "EGL_NV_stream_socket"
        "EGL_NV_stream_socket_unix"
        "EGL_NV_stream_socket_inet"
        "EGL_EXT_compositor"
        "EGL_EXT_surface_CTA861_3_metadata"
        "EGL_EXT_gl_colorspace_display_p3"
        "EGL_EXT_gl_colorspace_display_p3_linear"
        "EGL_EXT_gl_colorspace_scrgb"
        "EGL_EXT_image_implicit_sync_control"
        "EGL_EXT_bind_to_front"
        "EGL_ANDROID_get_frame_timestamps"
        "EGL_ANDROID_get_native_client_buffer"
        "EGL_NV_context_priority_realtime"
    );

    if extension_strings.len() > 0 || cfg!(feature = "extensions_module") {
        println!("rustc-cfg=extensions_module");

        let file_path = Path::new(&out_dir).join("egl_generated_extension_bindings.rs");
        generate_bindings(file_path, LATEST_EGL_VERSION, GlobalGenerator, extension_strings);
    }
}


fn generate_bindings<P: AsRef<Path>, G: Generator>(file_path: P, egl_version: (u8, u8), generator: G, egl_extensions: &[&str]) {
    let mut file = File::create(&file_path).unwrap();

    file.write(FILE_BEGINNING_TEXT).unwrap();

    Registry::new(Api::Egl, egl_version, Profile::Core, Fallbacks::All, egl_extensions)
        .write_bindings(generator, &mut file)
        .unwrap();
}