#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use bondage::*;
struct Foo {
    bar: String,
    baz: f64,
}
#[automatically_derived]
impl ::core::fmt::Debug for Foo {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Foo",
            "bar",
            &self.bar,
            "baz",
            &&self.baz,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Foo {
    #[inline]
    fn clone(&self) -> Foo {
        Foo {
            bar: ::core::clone::Clone::clone(&self.bar),
            baz: ::core::clone::Clone::clone(&self.baz),
        }
    }
}
#[automatically_derived]
impl Transferable<JsObject> for Foo {
    fn to_js<'cx>(&self, ctx: &mut Cx<'cx>) -> NeonResult<Handle<'cx, JsObject>> {
        let object = ctx.empty_object();
        let bar = self.bar.to_js(ctx)?;
        object.set(ctx, "bar", bar)?;
        let baz = self.baz.to_js(ctx)?;
        object.set(ctx, "baz", baz)?;
        Ok(object)
    }
    fn from_js<'cx>(
        ctx: &mut Cx<'cx>,
        object: Handle<'cx, JsObject>,
    ) -> NeonResult<Self> {
        let bar = object.get(ctx, "bar")?;
        let bar = Transferable::from_js(ctx, bar)?;
        let baz = object.get(ctx, "baz")?;
        let baz = Transferable::from_js(ctx, baz)?;
        Ok(Self { bar, baz })
    }
}
struct FooEvent {
    foo: Foo,
}
impl bondage::Event for FooEvent {
    fn name(&self) -> &str {
        "foo"
    }
    fn data<'cx>(&self, ctx: &mut Cx<'cx>) -> NeonResult<Vec<Handle<'cx, JsValue>>> {
        Ok(
            <[_]>::into_vec(
                ::alloc::boxed::box_new([self.foo.to_js(ctx)?.as_value(ctx)]),
            ),
        )
    }
}
struct BarEvent {
    bar: f64,
    nested: Foo,
}
impl bondage::Event for BarEvent {
    fn name(&self) -> &str {
        "bar"
    }
    fn data<'cx>(&self, ctx: &mut Cx<'cx>) -> NeonResult<Vec<Handle<'cx, JsValue>>> {
        Ok(
            <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    self.bar.to_js(ctx)?.as_value(ctx),
                    self.nested.to_js(ctx)?.as_value(ctx),
                ]),
            ),
        )
    }
}
#[used]
#[unsafe(link_section = "linkme_JS_EXPORTS")]
static JS_EXPORTS_createFoo: (&'static str, fn(FunctionContext) -> JsResult<JsValue>) = {
    #[allow(clippy::no_effect_underscore_binding)]
    unsafe fn __typecheck(_: ::linkme::__private35::Void) {
        #[allow(clippy::ref_option_ref)]
        let __new = || -> fn() -> &'static (
            &'static str,
            fn(FunctionContext) -> JsResult<JsValue>,
        ) { || &JS_EXPORTS_createFoo };
        unsafe {
            ::linkme::DistributedSlice::private_typecheck(JS_EXPORTS, __new());
        }
    }
    (
        "createFoo",
        |mut arg: FunctionContext| {
            create_foo(&mut arg)
                .and_then(|ret| ret.to_js(&mut arg))
                .map(|ret| ret.upcast())
        },
    )
};
fn create_foo(ctx: &mut FunctionContext) -> NeonResult<Foo> {
    let foo = Foo {
        bar: "foobar".to_string(),
        baz: 420f64,
    };
    Ok(foo)
}
#[used]
#[unsafe(link_section = "linkme_JS_EXPORTS")]
static JS_EXPORTS_triggerFooEvent: (
    &'static str,
    fn(FunctionContext) -> JsResult<JsValue>,
) = {
    #[allow(clippy::no_effect_underscore_binding)]
    unsafe fn __typecheck(_: ::linkme::__private35::Void) {
        #[allow(clippy::ref_option_ref)]
        let __new = || -> fn() -> &'static (
            &'static str,
            fn(FunctionContext) -> JsResult<JsValue>,
        ) { || &JS_EXPORTS_triggerFooEvent };
        unsafe {
            ::linkme::DistributedSlice::private_typecheck(JS_EXPORTS, __new());
        }
    }
    (
        "triggerFooEvent",
        |mut arg: FunctionContext| {
            trigger_foo_event(&mut arg)
                .and_then(|ret| ret.to_js(&mut arg))
                .map(|ret| ret.upcast())
        },
    )
};
fn trigger_foo_event(ctx: &mut FunctionContext) -> NeonResult<f64> {
    EVENT_SYSTEM
        .dispatch_event(FooEvent {
            foo: Foo {
                bar: "foobar".to_string(),
                baz: 69f64,
            },
        });
    Ok(65f64)
}
fn main(mut ctx: ModuleContext) -> NeonResult<()> {
    {
        #[used]
        #[unsafe(link_section = "linkme_MAIN")]
        #[allow(non_upper_case_globals)]
        static _LINKME_ELEMENT___NEON_MAIN__main: fn(
            cx: neon::context::ModuleContext,
        ) -> neon::result::NeonResult<()> = {
            #[allow(clippy::no_effect_underscore_binding)]
            unsafe fn __typecheck(_: neon::macro_internal::linkme::__private35::Void) {
                #[allow(clippy::ref_option_ref)]
                let __new = || -> fn() -> &'static fn(
                    cx: neon::context::ModuleContext,
                ) -> neon::result::NeonResult<()> {
                    || &_LINKME_ELEMENT___NEON_MAIN__main
                };
                unsafe {
                    neon::macro_internal::linkme::DistributedSlice::private_typecheck(
                        neon::macro_internal::MAIN,
                        __new(),
                    );
                }
            }
            __NEON_MAIN__main
        };
        fn __NEON_MAIN__main(
            cx: neon::context::ModuleContext,
        ) -> neon::result::NeonResult<()> {
            main(cx)
        }
    }
    {
        let event_system = EVENT_SYSTEM.lock().unwrap();
        event_system
            .set(EventSystem::new(ctx.channel()))
            .expect("Need to be able to initialize EventSystem");
        drop(event_system);
        for (name, function) in JS_EXPORTS {
            ctx.export_function(name, function)?;
        }
        ctx.export_function(
            "on",
            |mut ctx: FunctionContext| {
                let event = ctx.argument::<JsString>(0)?.value(&mut ctx);
                let callback = ctx.argument::<JsFunction>(1)?.root(&mut ctx);
                EVENT_SYSTEM.add_event_listener(event, callback);
                Ok(ctx.undefined())
            },
        )?;
        {}
        Ok(())
    }
}
