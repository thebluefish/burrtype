import {Bar, Enum, UnitStruct} from "./api/things"
import axios, {AxiosError} from "axios"
import {DeepTupleStruct} from "./api/deep";
import {Foo} from "./api/common";
import {NamedStruct, TupleStruct} from "./api/types";
import {assert_eq} from "./util"

let client = axios.create({
    baseURL: 'http://127.0.0.1:3000',
    headers: {
        'Content-Type': 'application/json'
    }
});

// wrap up main logic in async
run_all().catch((err: AxiosError) => {
    // attempt to log the url, message, and code for any http errors
    console.error(`"${err.response?.request.path}" ${err.message}: ${err.code}`)
})
.catch(err => {
    console.error(err)
})

async function run_all() {
    await foo()
    await bar()
    await deep_tuple_struct()
    await named_struct()
    await tuple_struct()
    await unit_struct()
    await enum_struct()
    await enum_tuple()
    await enum_tiny_tuple()
    await enum_unit()
    await enum_big_struct()
}

async function foo() {
    let result = await client.get<Foo>('/foo')
    console.log("foo: ", result.data)

    let data: Foo = { one: 1, two: "owt" }

    let ret = await client.post<Foo>('/foo', data)
    assert_eq(data, ret.data)
}

async function bar() {
    let result = await client.get<Bar>('/bar')
    console.log("bar: ", result.data)

    let data: Bar = { one: 2, two: "eno" }

    let ret = await client.post<Bar>('/bar', data)
    assert_eq(data, ret.data)
}

async function deep_tuple_struct() {
    let result = await client.get<DeepTupleStruct>('/deep_tuple_struct')
    console.log("deep_tuple_struct: ", result.data)

    let data: any = 1080

    let ret = await client.post<DeepTupleStruct>('/deep_tuple_struct', data)
    assert_eq(data, ret.data)
}

async function named_struct() {
    let result = await client.get<NamedStruct>('/named_struct')
    console.log("named_struct: ", result.data)

    let data: NamedStruct = {
        foo: 4,
        ty: 420.69
    }

    let ret = await client.post<NamedStruct>('/named_struct', data)
    assert_eq(data, ret.data)
}

async function tuple_struct() {
    let result = await client.get<TupleStruct>('/tuple_struct')
    console.log("tuple_struct: ", result.data)

    let data: TupleStruct = [69, { one: 420, two: "nice" }]

    let ret = await client.post<TupleStruct>('/tuple_struct', data)
    assert_eq(data, ret.data)
}

async function unit_struct() {
    let result = await client.get<UnitStruct>('/unit_struct')
    console.log("unit_struct: ", result.data)

    let data: UnitStruct = null

    let ret = await client.post<UnitStruct>('/unit_struct', data)
    assert_eq(data, ret.data)
}

async function enum_struct() {
    let result = await client.get<Enum>('/enum_struct')
    console.log("enum_struct: ", result.data)

    let data: Enum = {
        Struct: {
            foo: {
                one: 4,
                two: "eight",
            },
            bar: "sixteen"
        }
    }

    let ret = await client.post<Enum>('/enum_struct', data)
    assert_eq(data, ret.data)
}

async function enum_tuple() {
    let result = await client.get<Enum>('/enum_tuple')
    console.log("enum_tuple: ", result.data)

    let data: Enum = {
        Tuple: ["One", "Two"],
    }

    let ret = await client.post<Enum>('/enum_tuple', data)
    assert_eq(data, ret.data)
}

async function enum_tiny_tuple() {
    let result = await client.get<Enum>('/enum_tiny_tuple')
    console.log("enum_tiny_tuple: ", result.data)

    let data: Enum = {
        TinyTuple: "lol",
    }

    let ret = await client.post<Enum>('/enum_tiny_tuple', data)
    assert_eq(data, ret.data)
}

async function enum_unit() {
    let result = await client.get<Enum>('/enum_unit')
    console.log("enum_unit: ", result.data)

    let data: Enum = "Unit"

    let ret = await client.post<Enum>('/enum_unit', data)
    assert_eq(data, ret.data)
}

async function enum_big_struct() {
    let result = await client.get<Enum>('/enum_big_struct')
    console.log("enum_big_struct: ", result.data)

    let data: Enum = {
        BigStruct: {
            one: {
                one: 0,
                two: "1",
            },
            three: 3,
            five: [0, {one: 1, two: "2"}],
        }
    }

    let ret = await client.post<Enum>('/enum_big_struct', data)
    assert_eq(data, ret.data)
}
