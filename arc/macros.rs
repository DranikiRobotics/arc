/// A macro for simplifying the creation of a python module.
///
/// This exists because the `pyo3`'s `#[pymodule]` macro doesn't allow for the creation of submodules.
///
/// See [this](https://github.com/DranikiRobotics/arc/issues/3) for more information.
///
/// # Example
///
/// ```rust
/// # use pyo3::prelude::*;
/// fn my_submodule(py: Python<'_>, m: &PyModule) -> PyResult<()> {
///    m.add("true", true)?;
///    Ok(())
/// }
///
/// #[pymodule]
/// fn my_module(py: Python<'_>, m: &PyModule) -> PyResult<()> {
///    arc_pylib::pymod!(submodule -> my_submodule, "my_module.submodule", py, m);
///    Ok(())
/// }
/// ```
#[macro_export]
macro_rules! pymod {
    // Explanation:
    //
    // 1. We create the submodule via PyModule::new
    // 2. Call that the function that represents the submodule (providing py, so this module)
    // 3. Then we add the submodule to the parent module. (m.add_submodule(module_name))
    // 4. Finally, run a python script to add the submodule to sys.modules.
    ($module_name: ident -> $submodule_func: path, $module_path: literal, $py:ident, $m: ident) => {
        let $module_name = PyModule::new($py, ::core::stringify!($module_name))?;
        $submodule_func($py, $module_name)?;
        $m.add_submodule($module_name)?;
        pyo3::py_run!(
            $py,
            $module_name,
            ::core::concat!(
                "import sys; sys.modules['",
                $module_path,
                "'] = ",
                ::core::stringify!($module_name),
            )
        );
    };
}

/// A macro for simplifying the creation of a wrapped component.
///
/// This works for any component that implements the [`PyWrappedComponent`] trait.
///
/// # Example
///
/// ```rust
/// # use pyo3::prelude::*;
/// # use arc_pylib as pylib;
/// # use arc_pylib::arc_robot_hardware as hardware;
/// # use pylib::PyWrappedComponent as _;
/// # use pylib::__init__::hardware::gamepad::Gamepad;
/// # use pylib::__init__::Op;
///
/// // Instead of this:
/// let gamepad = Gamepad::new(hardware::gamepad::impls::LogitechF310::default());
/// let gamepad_wrapper = Gamepad::wrap(&gamepad);
/// let op = Op::new(gamepad_wrapper);
/// let op_wrapper = Op::wrap(&op);
///
/// // You can do this:
/// let (gamepad, gamepad_wrapper) = pylib::setup_wrapped_component!(
///     hardware::gamepad::impls::LogitechF310::default(); Gamepad
/// );
/// let (op, op_wrapper) = pylib::setup_wrapped_component!(gamepad_wrapper; Op);
/// ```
///
/// [`PyWrappedComponent`]: trait.PyWrappedComponent.html
#[macro_export]
macro_rules! setup_wrapped_component {
    (
        $value: expr; $component: ty
    ) => {{
        fn __setup_wrapped_component_internal<
            // Wrapped component constructor input
            Input,
            // Holder type (this is what will be read from by the python thread)
            Holder,
            // Hardware component (wrapped)
            Component,
        >(
            // The hardware component constructor
            input: Input,
            // We return a tuple
        ) -> (
            // A thread safe holder
            $crate::ThreadSafe<Holder>,
            // The wrapped component
            Component,
            // Validation
        )
        where
            // Component must implement PyWrappedComponent
            Component: $crate::PyWrappedComponent<Input, Holder = Holder>,
        {
            // Create the holder
            let holder = Component::new(input);
            // Wrap the holder in a thread safe type
            let wrapper = Component::wrap(&holder);
            // Return the holder and the wrapped component
            (holder, wrapper)
        }
        // Call the internal function
        __setup_wrapped_component_internal::<_, _, $component>($value)
    }};
}

#[macro_export(local_inner_macros)]
macro_rules! pyimpl {
    ($strcut: ident { $($body: tt)* }) => (
        impl $strcut {
            $crate::pyimpl!(@INNER $($body)*);
        }
    );
    (@INNER) => (

    )
}
