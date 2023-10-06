use crate::post_echo;
use sandbox::Foo;
use sandbox::inner::bar::DeepTupleStruct;
use sandbox::inner::TupleStruct;
use sandbox::serde_test::*;
use axum::{Json, Router};
use axum::routing::get;

pub fn setup(router: Router) -> Router {
    router
        // untagged enum variant
        .route("/untagged_enum_struct", get(get_untagged_enum_struct).post(post_echo::<UntaggedEnum>))
        .route("/untagged_enum_tuple", get(get_untagged_enum_tuple).post(post_echo::<UntaggedEnum>))
        .route("/untagged_enum_tiny_tuple", get(get_untagged_enum_tiny_tuple).post(post_echo::<UntaggedEnum>))
        .route("/untagged_enum_unit", get(get_untagged_enum_unit).post(post_echo::<UntaggedEnum>))
        .route("/untagged_enum_big_struct", get(get_untagged_enum_big_struct).post(post_echo::<UntaggedEnum>))

        // adjacently-tagged enum variant
        .route("/adjacent_enum_struct", get(get_adjacent_enum_struct).post(post_echo::<AdjacentlyTaggedEnum>))
        .route("/adjacent_enum_tuple", get(get_adjacent_enum_tuple).post(post_echo::<AdjacentlyTaggedEnum>))
        .route("/adjacent_enum_tiny_tuple", get(get_adjacent_enum_tiny_tuple).post(post_echo::<AdjacentlyTaggedEnum>))
        .route("/adjacent_enum_unit", get(get_adjacent_enum_unit).post(post_echo::<AdjacentlyTaggedEnum>))
        .route("/adjacent_enum_big_struct", get(get_adjacent_enum_big_struct).post(post_echo::<AdjacentlyTaggedEnum>))

        // internally-tagged enum variant
        .route("/internal_enum_struct", get(get_internal_enum_struct).post(post_echo::<InternallyTaggedEnum>))
        .route("/internal_enum_unit", get(get_internal_enum_unit).post(post_echo::<InternallyTaggedEnum>))
        .route("/internal_enum_big_struct", get(get_internal_enum_big_struct).post(post_echo::<InternallyTaggedEnum>))
}

async fn get_untagged_enum_struct() -> Json<UntaggedEnum> {
    let data = UntaggedEnum::Struct {
        foo: Foo { one: 8, two: "16".to_string() },
        bar: "bar".to_string(),
    };

    Json(data)
}

async fn get_untagged_enum_tuple() -> Json<UntaggedEnum> {
    let data = UntaggedEnum::Tuple(Stuff::Two, Stuff::Red);

    Json(data)
}

async fn get_untagged_enum_tiny_tuple() -> Json<UntaggedEnum> {
    let data = UntaggedEnum::TinyTuple("kek".into());

    Json(data)
}

async fn get_untagged_enum_unit() -> Json<UntaggedEnum> {
    let data = UntaggedEnum::Unit;

    Json(data)
}

async fn get_untagged_enum_big_struct() -> Json<UntaggedEnum> {
    let data = UntaggedEnum::BigStruct {
        three: DeepTupleStruct(4),
        four: Some(NamedStruct {
            foo: Stuff::Red,
            opt: None,
            more: Foo {
                one: 42,
                two: "24".to_string(),
            },
        }),
        five: TupleStruct(8, Foo {
            one: 16,
            two: "32".to_string(),
        }),
    };

    Json(data)
}

async fn get_adjacent_enum_struct() -> Json<AdjacentlyTaggedEnum> {
    let data = AdjacentlyTaggedEnum::Struct {
        foo: Foo { one: 8, two: "16".to_string() },
        bar: "bar".to_string(),
    };

    Json(data)
}

async fn get_adjacent_enum_tuple() -> Json<AdjacentlyTaggedEnum> {
    let data = AdjacentlyTaggedEnum::Tuple(Stuff::Red, Stuff::Two);

    Json(data)
}

async fn get_adjacent_enum_tiny_tuple() -> Json<AdjacentlyTaggedEnum> {
    let data = AdjacentlyTaggedEnum::TinyTuple("kek".into());

    Json(data)
}

async fn get_adjacent_enum_unit() -> Json<AdjacentlyTaggedEnum> {
    let data = AdjacentlyTaggedEnum::Unit;

    Json(data)
}

async fn get_adjacent_enum_big_struct() -> Json<AdjacentlyTaggedEnum> {
    let data = AdjacentlyTaggedEnum::BigStruct {
        three: DeepTupleStruct(4),
        four: Some(NamedStruct {
            foo: Stuff::Red,
            opt: None,
            more: Foo {
                one: 42,
                two: "24".to_string(),
            },
        }),
        five: TupleStruct(8, Foo {
            one: 16,
            two: "32".to_string(),
        }),
    };

    Json(data)
}

async fn get_internal_enum_struct() -> Json<InternallyTaggedEnum> {
    let data = InternallyTaggedEnum::Struct {
        foo: Foo { one: 8, two: "16".to_string() },
        bar: "bar".to_string(),
    };

    Json(data)
}

async fn get_internal_enum_unit() -> Json<InternallyTaggedEnum> {
    let data = InternallyTaggedEnum::Unit;

    Json(data)
}

async fn get_internal_enum_big_struct() -> Json<InternallyTaggedEnum> {
    let data = InternallyTaggedEnum::BigStruct {
        more: Foo {
            one: 1,
            two: "2".to_string(),
        },
        three: DeepTupleStruct(4),
        four: Some(NamedStruct {
            foo: Stuff::Red,
            opt: None,
            more: Foo {
                one: 42,
                two: "24".to_string(),
            },
        }),
        five: TupleStruct(8, Foo {
            one: 16,
            two: "32".to_string(),
        }),
    };

    Json(data)
}