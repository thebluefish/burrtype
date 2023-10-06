mod serde_test;

use sandbox::{*, inner::{*, bar::DeepTupleStruct}};
use std::net::SocketAddr;
use axum::{Json, Router, routing::get};
use rust_decimal::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/foo", get(get_foo).post(post_echo::<Foo>))
        .route("/bar", get(get_bar).post(post_echo::<Bar>))
        .route("/deep_tuple_struct", get(get_deep_tuple_struct).post(post_echo::<DeepTupleStruct>))
        .route("/named_struct", get(get_named_struct).post(post_echo::<NamedStruct>))
        .route("/tuple_struct", get(get_tuple_struct).post(post_echo::<TupleStruct>))
        .route("/unit_struct", get(get_unit_struct).post(post_echo::<UnitStruct>))
        .route("/enum_struct", get(get_enum_struct).post(post_echo::<Enum>))
        .route("/enum_tuple", get(get_enum_tuple).post(post_echo::<Enum>))
        .route("/enum_tiny_tuple", get(get_enum_tiny_tuple).post(post_echo::<Enum>))
        .route("/enum_unit", get(get_enum_unit).post(post_echo::<Enum>))
        .route("/enum_big_struct", get(get_enum_big_struct).post(post_echo::<Enum>))
    ;

    let app = serde_test::setup(app);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn post_echo<T: std::fmt::Debug>(input: Json<T>) -> Json<T> {
    println!("got {:#?}", input.0);
    input
}

async fn get_foo() -> Json<Foo> {
    let data = Foo {
        one: 65536,
        two: "2^16".to_string(),
    };

    Json(data)
}

async fn get_bar() -> Json<Bar> {
    let data = Bar(Foo {
        one: 3,
        two: "6".to_string(),
    });

    Json(data)
}

async fn get_deep_tuple_struct() -> Json<DeepTupleStruct> {
    let data = DeepTupleStruct(64);

    Json(data)
}

async fn get_named_struct() -> Json<NamedStruct> {
    let data = NamedStruct {
        foo: PhantomType(42),
        ty: Decimal::from_str("1.32").unwrap(),
        opt: Some(Foo {
            one: 1,
            two: "2".to_string(),
        }),
    };

    Json(data)
}

async fn get_tuple_struct() -> Json<TupleStruct> {
    let data = TupleStruct(32, Foo {
        one: 11,
        two: "22".to_string(),
    });

    Json(data)
}

async fn get_unit_struct() -> Json<UnitStruct> {
    let data = UnitStruct;

    Json(data)
}

async fn get_enum_struct() -> Json<Enum> {
    let data = Enum::Struct {
        foo: Foo { one: 8, two: "16".to_string() },
        bar: "bar".to_string(),
    };

    Json(data)
}

async fn get_enum_tuple() -> Json<Enum> {
    let data = Enum::Tuple(Things::Two, Things::One);

    Json(data)
}

async fn get_enum_tiny_tuple() -> Json<Enum> {
    let data = Enum::TinyTuple("kek".into());

    Json(data)
}

async fn get_enum_unit() -> Json<Enum> {
    let data = Enum::Unit;

    Json(data)
}

async fn get_enum_big_struct() -> Json<Enum> {
    let data = Enum::BigStruct {
        one: Foo {
            one: 6,
            two: "12".to_string(),
        },
        three: DeepTupleStruct(4),
        four: Some(NamedStruct {
            foo: PhantomType(6),
            ty: Decimal::new(132, 2),
            opt: None,
        }),
        five: TupleStruct(8, Foo {
            one: 16,
            two: "32".to_string(),
        }),
    };

    Json(data)
}
