import {Bar, Enum, UnitStruct} from "./api/things"
import {DeepTupleStruct, Foo, NamedStruct, TupleStruct} from "./api/common"
import axios, {AxiosError} from "axios"

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
    let result = await client.get<DeepTupleStruct>('/foo')
    console.log("foo: ", result.data)

    let data: Foo = { one: 1, two: "owt" }

    await client.post('/foo', data)
}

async function bar() {
    let result = await client.get<Bar>('/bar')
    console.log("bar: ", result.data)

    let data: Bar = { one: 2, two: "eno" }

    await client.post('/bar', data)
}

async function deep_tuple_struct() {
    let result = await client.get<DeepTupleStruct>('/deep_tuple_struct')
    console.log("deep_tuple_struct: ", result.data)

    let data: any = 1080

    await client.post('/deep_tuple_struct', data)
}

async function named_struct() {
    let result = await client.get<NamedStruct>('/named_struct')
    console.log("named_struct: ", result.data)

    let data: NamedStruct = {
        foo: 4,
    }

    await client.post('/named_struct', data)
}

async function tuple_struct() {
    let result = await client.get<TupleStruct>('/tuple_struct')
    console.log("tuple_struct: ", result.data)

    let data: TupleStruct = [69, { one: 420, two: "nice" }]

    await client.post('/tuple_struct', data)
}

async function unit_struct() {
    let result = await client.get('/unit_struct')
    console.log("unit_struct: ", result.data)

    let data: UnitStruct = null

    await client.post('/unit_struct', data)
}

async function enum_struct() {
    let result = await client.get('/enum_struct')
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

    await client.post('/enum_struct', data)
}

async function enum_tuple() {
    let result = await client.get('/enum_tuple')
    console.log("enum_tuple: ", result.data)

    let data: Enum = {
        Tuple: [16, 32],
    }

    await client.post('/enum_tuple', data)
}

async function enum_tiny_tuple() {
    let result = await client.get('/enum_tiny_tuple')
    console.log("enum_tiny_tuple: ", result.data)

    let data: Enum = {
        TinyTuple: "lol",
    }

    await client.post('/enum_tiny_tuple', data)
}

async function enum_unit() {
    let result = await client.get('/enum_unit')
    console.log("enum_unit: ", result.data)

    let data: Enum = "Unit"

    await client.post('/enum_unit', data)
}

async function enum_big_struct() {
    let result = await client.get('/enum_big_struct')
    console.log("enum_big_struct: ", result.data)

    let data: Enum = {
        BigStruct: {
            one: 1,
            three: [3, { one: 6, two: "12" }],
            four:  { one: 1, two: "2" },
        }
    }

    await client.post('/enum_big_struct', data)
}
