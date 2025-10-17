use bondage::*;

#[derive(Debug, Clone, Transferable)]
struct Foo {
    bar: String,
    baz: f64,
}

#[derive(Event)]
struct FooEvent {
    foo: Foo,
}

#[derive(Event)]
struct BarEvent {
    bar: f64,
    nested: Foo
}

#[export]
fn create_foo(ctx: &mut FunctionContext) -> NeonResult<Foo> {
    let foo = Foo {
        bar: "foobar".to_string(),
        baz: 420f64,
    };
    Ok(foo)
}

#[export]
fn trigger_foo_event(ctx: &mut FunctionContext) -> NeonResult<f64> {
    EVENT_SYSTEM.dispatch_event(FooEvent {
        foo: Foo {
            bar: "foobar".to_string(),
            baz: 69f64,
        },
    });


    Ok(65f64)
}

#[main]
fn main(mut ctx: ModuleContext) {
    //any startup code goes here
}
