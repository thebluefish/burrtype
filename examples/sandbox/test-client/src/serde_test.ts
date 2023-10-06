import { AxiosInstance } from "axios"
import {AdjacentlyTaggedEnum, InternallyTaggedEnum, UntaggedEnum} from "./api/things"
import {assert_eq} from "./util";

export default async function run_all(client: AxiosInstance) {
    // untagged enum variant
    await untagged_enum_struct(client)
    await untagged_enum_tuple(client)
    await untagged_enum_tiny_tuple(client)
    await untagged_enum_unit(client)
    await untagged_enum_big_struct(client)

    // adjacently-tagged enum variant
    await adjacent_enum_struct(client)
    await adjacent_enum_tuple(client)
    await adjacent_enum_tiny_tuple(client)
    await adjacent_enum_unit(client)
    await adjacent_enum_big_struct(client)

    // internally-tagged enum variant
    await internal_enum_struct(client)
    await internal_enum_unit(client)
    await internal_enum_big_struct(client)

}

async function untagged_enum_struct(client: AxiosInstance) {
    let result = await client.get<UntaggedEnum>('/untagged_enum_struct')
    console.log("untagged_enum_struct: ", result.data)

    let data: UntaggedEnum = {
        foo: {
            one: 4,
            two: "eight",
        },
        bar: "sixteen",
    }

    let ret = await client.post<UntaggedEnum>('/untagged_enum_struct', data)
    assert_eq(data, ret.data)
}

async function untagged_enum_tuple(client: AxiosInstance) {
    let result = await client.get<UntaggedEnum>('/untagged_enum_tuple')
    console.log("untagged_enum_tuple: ", result.data)

    let data: UntaggedEnum = ["red", "two"]

    let ret = await client.post<UntaggedEnum>('/untagged_enum_tuple', data)
    assert_eq(data, ret.data)
}

async function untagged_enum_tiny_tuple(client: AxiosInstance) {
    let result = await client.get<UntaggedEnum>('/untagged_enum_tiny_tuple')
    console.log("untagged_enum_tiny_tuple: ", result.data)

    let data: UntaggedEnum = "lol"

    let ret = await client.post<UntaggedEnum>('/untagged_enum_tiny_tuple', data)
    assert_eq(data, ret.data)
}

async function untagged_enum_unit(client: AxiosInstance) {
    let result = await client.get<UntaggedEnum>('/untagged_enum_unit')
    console.log("untagged_enum_unit: ", result.data)

    let data: UntaggedEnum = "unit"

    let ret = await client.post<UntaggedEnum>('/untagged_enum_unit', data)
    assert_eq(data, ret.data)
}

async function untagged_enum_big_struct(client: AxiosInstance) {
    let result = await client.get<UntaggedEnum>('/untagged_enum_big_struct')
    console.log("untagged_enum_big_struct: ", result.data)

    let data: UntaggedEnum = {
        THREE: 3,
        six: [0, {one: 1, two: "2"}],
    }

    let ret = await client.post<UntaggedEnum>('/untagged_enum_big_struct', data)
    assert_eq(data, ret.data)
}

async function adjacent_enum_struct(client: AxiosInstance) {
    let result = await client.get<AdjacentlyTaggedEnum>('/adjacent_enum_struct')
    console.log("adjacent_enum_struct: ", result.data)

    let data: AdjacentlyTaggedEnum = {
        t: "Struct",
        c: {
            foo: {
                one: 4,
                two: "eight",
            },
            bar: "sixteen"
        }
    }

    let ret = await client.post<AdjacentlyTaggedEnum>('/adjacent_enum_struct', data)
    assert_eq(data, ret.data)
}

async function adjacent_enum_tuple(client: AxiosInstance) {
    let result = await client.get<AdjacentlyTaggedEnum>('/adjacent_enum_tuple')
    console.log("adjacent_enum_tuple: ", result.data)

    let data: AdjacentlyTaggedEnum = {
        t: "Tuple",
        c: ["red", "two"],
    }

    let ret = await client.post<AdjacentlyTaggedEnum>('/adjacent_enum_tuple', data)
    assert_eq(data, ret.data)
}

async function adjacent_enum_tiny_tuple(client: AxiosInstance) {
    let result = await client.get<AdjacentlyTaggedEnum>('/adjacent_enum_tiny_tuple')
    console.log("adjacent_enum_tiny_tuple: ", result.data)

    let data: AdjacentlyTaggedEnum = {
        t: "TinyTuple",
        c: "lol",
    }

    let ret = await client.post<AdjacentlyTaggedEnum>('/adjacent_enum_tiny_tuple', data)
    assert_eq(data, ret.data)
}

async function adjacent_enum_unit(client: AxiosInstance) {
    let result = await client.get<AdjacentlyTaggedEnum>('/adjacent_enum_unit')
    console.log("adjacent_enum_unit: ", result.data)

    let data: AdjacentlyTaggedEnum = { t: "Unit" }

    let ret = await client.post<AdjacentlyTaggedEnum>('/adjacent_enum_unit', data)
    assert_eq(data, ret.data)
}

async function adjacent_enum_big_struct(client: AxiosInstance) {
    let result = await client.get<AdjacentlyTaggedEnum>('/adjacent_enum_big_struct')
    console.log("adjacent_enum_big_struct: ", result.data)

    let data: AdjacentlyTaggedEnum = {
        t: "BigStruct",
        c: {
            THREE: 3,
            six: [0, {one: 1, two: "2"}],
        }
    }

    let ret = await client.post<AdjacentlyTaggedEnum>('/adjacent_enum_big_struct', data)
    assert_eq(data, ret.data)
}

async function internal_enum_struct(client: AxiosInstance) {
    let result = await client.get<InternallyTaggedEnum>('/internal_enum_struct')
    console.log("internal_enum_struct: ", result.data)

    let data: InternallyTaggedEnum = {
        type: "Struct",
        foo: {
            one: 4,
            two: "eight",
        },
        bar: "sixteen"
    }

    let ret = await client.post<InternallyTaggedEnum>('/internal_enum_struct', data)
    assert_eq(data, ret.data)
}

async function internal_enum_unit(client: AxiosInstance) {
    let result = await client.get<InternallyTaggedEnum>('/internal_enum_unit')
    console.log("internal_enum_unit: ", result.data)

    let data: InternallyTaggedEnum = { type: "Unit" }

    let ret = await client.post<InternallyTaggedEnum>('/internal_enum_unit', data)
    assert_eq(data, ret.data)
}

async function internal_enum_big_struct(client: AxiosInstance) {
    let result = await client.get<InternallyTaggedEnum>('/internal_enum_big_struct')
    console.log("internal_enum_big_struct: ", result.data)

    let data: InternallyTaggedEnum = {
        type: "BigStruct",
        one: 1,
        two: "two",
        THREE: 3,
        six: [0, {one: 1, two: "2"}],
    }

    let ret = await client.post<InternallyTaggedEnum>('/internal_enum_big_struct', data)
    assert_eq(data, ret.data)
}
