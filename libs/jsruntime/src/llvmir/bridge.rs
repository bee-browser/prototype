#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::types;
use crate::VoidPtr;

include!(concat!(env!("OUT_DIR"), "/bridge.rs"));

macro_rules! into_runtime {
    ($runtime:expr, $extension:ident) => {
        &mut *($runtime as *mut crate::Runtime<$extension>)
    };
}

macro_rules! into_value {
    ($value:expr) => {
        // TODO: remove type cast
        &*($value as *const crate::types::Value)
    };
}

pub fn runtime_bridge<X>() -> Runtime {
    Runtime {
        to_boolean: Some(runtime_to_boolean),
        to_numeric: Some(runtime_to_numeric),
        to_int32: Some(runtime_to_int32),
        to_uint32: Some(runtime_to_uint32),
        is_loosely_equal: Some(runtime_is_loosely_equal),
        is_strictly_equal: Some(runtime_is_strictly_equal),
        create_capture: Some(runtime_create_capture::<X>),
        create_closure: Some(runtime_create_closure::<X>),
        create_coroutine: Some(runtime_create_coroutine::<X>),
        register_promise: Some(runtime_register_promise::<X>),
        await_promise: Some(runtime_await_promise::<X>),
        resume: Some(runtime_resume::<X>),
        emit_promise_resolved: Some(runtime_emit_promise_resolved::<X>),
        assert: Some(runtime_assert),
        print_u32: Some(runtime_print_u32),
        print_f64: Some(runtime_print_f64),
        print_value: Some(runtime_print_value),
        launch_debugger: Some(runtime_launch_debugger),
    }
}

// 7.1.2 ToBoolean ( argument )
unsafe extern "C" fn runtime_to_boolean(_runtime: VoidPtr, value: *const Value) -> bool {
    let value = into_value!(value);
    match value {
        types::Value::None => unreachable!("Value::None"),
        types::Value::Undefined => false,
        types::Value::Null => false,
        types::Value::Boolean(value) => *value,
        types::Value::Number(value) if *value == 0.0 => false,
        types::Value::Number(value) if value.is_nan() => false,
        types::Value::Number(_) => true,
        types::Value::Closure(_) => true,
        types::Value::Promise(_) => true,
    }
}

// 7.1.3 ToNumeric ( value )
// 7.1.4 ToNumber ( argument )
unsafe extern "C" fn runtime_to_numeric(_runtime: VoidPtr, value: *const Value) -> f64 {
    let value = into_value!(value);
    match value {
        types::Value::None => unreachable!("Value::None"),
        types::Value::Undefined => f64::NAN,
        types::Value::Null => 0.0,
        types::Value::Boolean(value) if *value => 1.0,
        types::Value::Boolean(_) => 0.0,
        types::Value::Number(value) => *value,
        types::Value::Closure(_) => f64::NAN,
        types::Value::Promise(_) => f64::NAN,
    }
}

// 7.1.6 ToInt32 ( argument )
unsafe extern "C" fn runtime_to_int32(_runtime: VoidPtr, value: f64) -> i32 {
    const EXP2_31: f64 = (2u64 << 31) as f64;
    const EXP2_32: f64 = (2u64 << 32) as f64;

    // 2. If number is not finite or number is either +0𝔽 or -0𝔽, return +0𝔽.
    if !value.is_finite() || value == 0.0 {
        return 0;
    }

    // 3. Let int be truncate(ℝ(number)).
    let int_ = value.trunc();

    // 4. Let int32bit be int modulo 2**32.
    let int32bit = int_ % EXP2_32;
    // int32bit may be negative.

    // 5. If int32bit ≥ 2**31, return 𝔽(int32bit - 2**32); otherwise return 𝔽(int32bit).
    if int32bit >= EXP2_31 {
        (int32bit - EXP2_32) as i32
    } else {
        int32bit as i32
    }
}

// 7.1.7 ToUint32 ( argument )
unsafe extern "C" fn runtime_to_uint32(_runtime: VoidPtr, value: f64) -> u32 {
    const EXP2_31: f64 = (2u64 << 31) as f64;
    const EXP2_32: f64 = (2u64 << 32) as f64;

    // 2. If number is not finite or number is either +0𝔽 or -0𝔽, return +0𝔽.
    if !value.is_finite() || value == 0.0 {
        return 0;
    }

    // 3. Let int be truncate(ℝ(number)).
    let int_ = dbg!(value.trunc());

    // 4. Let int32bit be int modulo 2**32.
    let int32bit = dbg!(int_ % EXP2_32);
    // int32bit may be negative.

    // 5. Return 𝔽(int32bit).
    if int32bit < 0.0 {
        dbg!((int32bit + EXP2_31) as u32)
    } else {
        dbg!(int32bit as u32)
    }
}

// 7.2.13 IsLooselyEqual ( x, y )
unsafe extern "C" fn runtime_is_loosely_equal(
    runtime: VoidPtr,
    a: *const Value,
    b: *const Value,
) -> bool {
    let x = into_value!(a);
    debug_assert!(!matches!(x, types::Value::None));

    let y = into_value!(b);
    debug_assert!(!matches!(y, types::Value::None));

    let x_kind = std::mem::discriminant(x);
    let y_kind = std::mem::discriminant(y);

    // 1. If Type(x) is Type(y)
    if x_kind == y_kind {
        // a. Return IsStrictlyEqual(x, y).
        return runtime_is_strictly_equal(runtime, a, b);
    }

    match (x, y) {
        // 2. If x is null and y is undefined, return true.
        (types::Value::Null, types::Value::Undefined) => true,
        // 3. If x is undefined and y is null, return true.
        (types::Value::Undefined, types::Value::Null) => true,
        // TODO: 4. NOTE: This step is replaced in section B.3.6.2.
        // TODO: 5. If x is a Number and y is a String, return ! IsLooselyEqual(x, ! ToNumber(y)).
        // TODO: 6. If x is a String and y is a Number, return ! IsLooselyEqual(! ToNumber(x), y).
        // TODO: 7. If x is a BigInt and y is a String, then
        // TODO: 8. If x is a String and y is a BigInt, return ! IsLooselyEqual(y, x).
        // TODO: 9. If x is a Boolean, return ! IsLooselyEqual(! ToNumber(x), y).
        // TODO: 10. If y is a Boolean, return ! IsLooselyEqual(x, ! ToNumber(y)).
        // ...
        _ => {
            let xnum = runtime_to_numeric(runtime, a);
            let ynum = runtime_to_numeric(runtime, b);
            if xnum.is_nan() || ynum.is_nan() {
                return false;
            }
            xnum == ynum
        }
    }
}

// 7.2.14 IsStrictlyEqual ( x, y )
unsafe extern "C" fn runtime_is_strictly_equal(
    _runtime: VoidPtr,
    a: *const Value,
    b: *const Value,
) -> bool {
    let x = into_value!(a);
    debug_assert!(!matches!(x, types::Value::None));

    let y = into_value!(b);
    debug_assert!(!matches!(y, types::Value::None));

    x == y
}

unsafe extern "C" fn runtime_create_capture<X>(
    runtime: VoidPtr,
    target: *mut Value,
) -> *mut Capture {
    const LAYOUT: std::alloc::Layout = unsafe {
        std::alloc::Layout::from_size_align_unchecked(
            std::mem::size_of::<types::Capture>(),
            std::mem::align_of::<types::Capture>(),
        )
    };

    let runtime = into_runtime!(runtime, X);
    let allocator = runtime.allocator();

    // TODO: GC
    let ptr = allocator.alloc_layout(LAYOUT);

    let capture = ptr.cast::<types::Capture>().as_ptr();
    // TODO: remove type cast
    (*capture).target = target as *mut types::Value;

    // `capture.escaped` will be filled with an actual value.

    // TODO: remove type cast
    capture as *mut types::Capture as *mut Capture
}

unsafe extern "C" fn runtime_create_closure<X>(
    runtime: VoidPtr,
    lambda: Lambda,
    num_captures: u16,
) -> *mut Closure {
    const BASE_LAYOUT: std::alloc::Layout = unsafe {
        std::alloc::Layout::from_size_align_unchecked(
            std::mem::offset_of!(types::Closure, captures),
            std::mem::align_of::<types::Closure>(),
        )
    };

    let storage_layout = std::alloc::Layout::array::<*mut types::Capture>(num_captures as usize).unwrap();
    let (layout, _) = BASE_LAYOUT.extend(storage_layout).unwrap();

    let runtime = into_runtime!(runtime, X);
    let allocator = runtime.allocator();

    // TODO: GC
    let ptr = allocator.alloc_layout(layout);

    let closure = ptr.cast::<types::Closure>().as_ptr();
    (*closure).lambda = lambda;
    (*closure).num_captures = num_captures;
    // `(*closure).captures[]` will be filled with actual pointers to `Captures`.

    // TODO: remove type cast
    closure as *mut types::Closure as *mut Closure
}

unsafe extern "C" fn runtime_create_coroutine<X>(
    runtime: VoidPtr,
    closure: *mut Closure,
    num_locals: u16,
    scratch_buffer_len: u16,
) -> *mut Coroutine {
    const BASE_LAYOUT: std::alloc::Layout = unsafe {
        std::alloc::Layout::from_size_align_unchecked(
            std::mem::offset_of!(types::Coroutine, locals),
            std::mem::align_of::<types::Coroutine>(),
        )
    };

    // num_locals may be 0.
    let locals_layout = std::alloc::Layout::array::<types::Value>(num_locals as usize).unwrap();
    let (layout, _) = BASE_LAYOUT.extend(locals_layout).unwrap();

    // scratch_buffer_len may be 0.
    debug_assert_eq!(scratch_buffer_len as usize % size_of::<u64>(), 0);
    let n = scratch_buffer_len as usize / size_of::<u64>();
    let scratch_buffer_layout = std::alloc::Layout::array::<u64>(n).unwrap();
    let (layout, _) = layout.extend(scratch_buffer_layout).unwrap();

    let runtime = into_runtime!(runtime, X);
    let allocator = runtime.allocator();

    // TODO: GC
    let ptr = allocator.alloc_layout(layout);

    let coroutine = ptr.cast::<types::Coroutine>().as_ptr();
    // TODO: remove type cast
    (*coroutine).closure = closure as *mut types::Closure;
    (*coroutine).state = 0;
    (*coroutine).num_locals = num_locals;
    (*coroutine).scope_id = 0;
    (*coroutine).scratch_buffer_len = scratch_buffer_len;
    // `(*coroutine).locals[]` will be initialized in the coroutine.

    // TODO: remove type cast
    coroutine as *mut types::Coroutine as *mut Coroutine
}

unsafe extern "C" fn runtime_register_promise<X>(
    runtime: VoidPtr,
    coroutine: *mut Coroutine,
) -> u32 {
    let runtime = into_runtime!(runtime, X);
    // TODO: remove type cast
    runtime.register_promise(coroutine as *mut types::Coroutine).into()
}

unsafe extern "C" fn runtime_resume<X>(runtime: VoidPtr, promise: u32) {
    let runtime = into_runtime!(runtime, X);
    runtime.process_promise(promise.into(), &types::Value::None, &types::Value::None);
}

unsafe extern "C" fn runtime_await_promise<X>(runtime: VoidPtr, promise: u32, awaiting: u32) {
    let runtime = into_runtime!(runtime, X);
    runtime.await_promise(promise.into(), awaiting.into());
}

unsafe extern "C" fn runtime_emit_promise_resolved<X>(
    runtime: VoidPtr,
    promise: u32,
    result: *const Value,
) {
    let runtime = into_runtime!(runtime, X);
    let cloned = into_value!(result).clone();
    runtime.emit_promise_resolved(promise.into(), cloned);
}

unsafe extern "C" fn runtime_assert(
    _runtime: VoidPtr,
    assertion: bool,
    msg: *const std::os::raw::c_char,
) {
    if !assertion {
        let msg = std::ffi::CStr::from_ptr(msg);
        panic!("runtime_assert: {msg:?}");
    }
}

unsafe extern "C" fn runtime_print_u32(
    _runtime: VoidPtr,
    value: u32,
    msg: *const std::os::raw::c_char,
) {
    let msg = std::ffi::CStr::from_ptr(msg);
    if msg.is_empty() {
        crate::logger::debug!("runtime_print_u32: {value:08X}");
    } else {
        crate::logger::debug!("runtime_print_u32: {value:08X}: {msg:?}");
    }
}

unsafe extern "C" fn runtime_print_f64(
    _runtime: VoidPtr,
    value: f64,
    msg: *const std::os::raw::c_char,
) {
    let msg = std::ffi::CStr::from_ptr(msg);
    if msg.is_empty() {
        crate::logger::debug!("runtime_print_f64: {value}");
    } else {
        crate::logger::debug!("runtime_print_f64: {value}: {msg:?}");
    }
}

unsafe extern "C" fn runtime_print_value(
    _runtime: VoidPtr,
    value: *const Value,
    msg: *const std::os::raw::c_char,
) {
    let value = into_value!(value);
    let msg = std::ffi::CStr::from_ptr(msg);
    if msg.is_empty() {
        crate::logger::debug!("runtime_print_value: {value:?}");
    } else {
        crate::logger::debug!("runtime_print_value: {value:?}: {msg:?}");
    }
}

unsafe extern "C" fn runtime_launch_debugger(_runtime: VoidPtr) {
    crate::logger::debug!("runtime_launch_debugger");
    // TODO(feat): Support debuggers such as Chrome DevTools.
}
