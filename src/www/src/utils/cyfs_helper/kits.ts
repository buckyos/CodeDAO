// cyfs scattered utility functions

import * as cyfs from 'cyfs-sdk';

export async function listObjects(
    stack: cyfs.SharedCyfsStack,
    ids: Array<cyfs.ObjectId>
): Promise<Array<cyfs.BuckyResult<cyfs.NONObjectInfo>>> {
    if (ids.length === 0) {
        return [];
    }

    const getR = await Promise.all(
        ids.map((id) =>
            stack.non_service().get_object({
                common: { level: cyfs.NONAPILevel.Router, flags: 0 },
                object_id: id
            })
        )
    );

    return getR.map((r) => {
        if (r.err) {
            return r;
        }
        return cyfs.Ok(r.unwrap().object);
    });
}

export function textObject(
    text: string,
    owner: cyfs.Option<cyfs.ObjectId> = cyfs.None
): cyfs.TextObject {
    return cyfs.TextObject.create(owner, text, text, text);
}

export interface AsObjectDescContent {}

export interface AsObjectDesc {
    trace_id: () => number;
    content: () => AsObjectDescContent;
    dec_id: () => cyfs.Option<cyfs.ObjectId>;
    ref_objs: () => cyfs.Option<cyfs.Vec<cyfs.ObjectLink>>;
    prev: () => cyfs.Option<cyfs.ObjectId>;
    create_timestamp: () => cyfs.Option<cyfs.HashValue>;
    create_time: () => cyfs.JSBI;
    expired_time: () => cyfs.Option<cyfs.JSBI>;
    object_id: () => cyfs.ObjectId;
    calculate_id: () => cyfs.ObjectId;
    owner: () => cyfs.Option<cyfs.ObjectId> | undefined;
    area: () => cyfs.Option<cyfs.Area> | undefined;
    author: () => cyfs.Option<cyfs.ObjectId> | undefined;
    public_key: () => cyfs.PublicKey | undefined;
    mn_key: () => cyfs.MNPublicKey | undefined;
    raw_measure: (
        ctx?: cyfs.NamedObjectContext,
        purpose?: cyfs.RawEncodePurpose
    ) => cyfs.BuckyResult<number>;
    raw_measure_with_context: (
        ctx: cyfs.NamedObjectContext,
        purpose?: cyfs.RawEncodePurpose
    ) => cyfs.BuckyResult<number>;
    raw_encode: (
        buf: Uint8Array,
        ctx?: cyfs.NamedObjectContext,
        purpose?: cyfs.RawEncodePurpose
    ) => cyfs.BuckyResult<Uint8Array>;
    raw_encode_with_context: (
        buf: Uint8Array,
        ctx: cyfs.NamedObjectContext,
        purpose?: cyfs.RawEncodePurpose
    ) => cyfs.BuckyResult<Uint8Array>;
    encode_to_buf: (purpose?: cyfs.RawEncodePurpose) => cyfs.BuckyResult<Uint8Array>;
    raw_hash_encode: () => cyfs.BuckyResult<cyfs.HashValue>;
}

export interface AsObjectBodyContent {}

export interface AsObjectBody {
    toString: () => string;
    set_trace_id: (trace: number) => void;
    trace_id: () => number | undefined;
    prev_version: () => cyfs.Option<cyfs.HashValue>;
    update_time: () => cyfs.JSBI;
    content: () => AsObjectBodyContent;
    user_data: () => cyfs.Option<Uint8Array>;
    set_update_time: (value: cyfs.JSBI) => void;
    increase_update_time: (value: cyfs.JSBI) => void;
    set_userdata: (user_data: Uint8Array) => void;
    raw_measure: (
        ctx: cyfs.NamedObjectBodyContext,
        purpose?: cyfs.RawEncodePurpose
    ) => cyfs.BuckyResult<number>;
    raw_encode: (
        buf: Uint8Array,
        ctx: cyfs.NamedObjectBodyContext,
        purpose?: cyfs.RawEncodePurpose
    ) => cyfs.BuckyResult<Uint8Array>;
    encode_to_buf: (purpose?: cyfs.RawEncodePurpose) => cyfs.BuckyResult<Uint8Array>;
    raw_hash_encode: () => cyfs.BuckyResult<cyfs.HashValue>;
}

export interface AsObject {
    obj_type: () => number;
    obj_type_code: () => number;
    calculate_id: () => cyfs.ObjectId;
    to_vec: () => cyfs.BuckyResult<Uint8Array>;
    to_hex: () => cyfs.BuckyResult<string>;
    toString: () => string;
    to_string: () => string;
    toJSON: () => string;
    desc: () => AsObjectDesc;
    body: () => cyfs.Option<AsObjectBody>;
    body_expect: () => AsObjectBody;
    signs: () => cyfs.ObjectSigns;
    nonce: () => cyfs.Option<cyfs.JSBI>;
    raw_measure: (_ctx?: any, purpose?: cyfs.RawEncodePurpose) => cyfs.BuckyResult<number>;
    raw_encode: (
        buf: Uint8Array,
        ctx?: cyfs.NamedObjectContext,
        purpose?: cyfs.RawEncodePurpose
    ) => cyfs.BuckyResult<Uint8Array>;
    encode_to_buf: (purpose?: cyfs.RawEncodePurpose) => cyfs.BuckyResult<Uint8Array>;
    raw_hash_encode: () => cyfs.BuckyResult<cyfs.HashValue>;
}

export interface AsDecoderFromRaw<T extends AsObject> {
    from_raw: (buf: Uint8Array) => cyfs.BuckyResult<T>;
}

export function toNONObjectInfo(obj: AsObject): cyfs.NONObjectInfo {
    return new cyfs.NONObjectInfo(obj.desc().object_id(), obj.encode_to_buf().unwrap());
}

export function fromNONObjectInfo<O extends AsObject>(
    nonObj: cyfs.NONObjectInfo,
    DecoderGen: new () => AsDecoderFromRaw<O>
): cyfs.BuckyResult<O> {
    return new DecoderGen().from_raw(nonObj.object_raw);
}

export function wrapPostResponseResult(
    r: cyfs.BuckyResult<cyfs.NONPostObjectInputResponse>
): cyfs.BuckyResult<cyfs.RouterHandlerPostObjectResult> {
    return cyfs.Ok({
        action: cyfs.RouterHandlerAction.Response,
        response: r
    });
}

export function makeBuckyErr(
    code: string | number | cyfs.BuckyErrorCodeEx,
    msg: string,
    origin?: string
): cyfs.Err<cyfs.BuckyError> {
    return cyfs.Err(new cyfs.BuckyError(code, msg, origin));
}

export function checkObjectId(id: Uint8Array): cyfs.ObjectId | undefined {
    if (id.length === cyfs.OBJECT_ID_LEN) {
        return cyfs.ObjectId.copy_from_slice(id);
    }
    return undefined;
}

export function checkObjectIdArray(ids: Array<Uint8Array>): {
    ids?: Array<cyfs.ObjectId>;
    errIndex?: number;
} {
    for (let i = 0; i < ids.length; i += 1) {
        if (ids[i].length !== cyfs.OBJECT_ID_LEN) {
            return { errIndex: i };
        }
    }

    return {
        ids: ids.map((b) => cyfs.ObjectId.copy_from_slice(b))
    };
}

/**
 * Check the peopleId array
 * @param ids
 * @returns
 */
export function checkPeopleIdArray(ids: Array<Uint8Array>): {
    ids?: Array<cyfs.PeopleId>;
    errIndex?: number;
} {
    for (let i = 0; i < ids.length; i += 1) {
        if (ids[i].length !== cyfs.OBJECT_ID_LEN) {
            return { errIndex: i };
        }
    }

    return {
        ids: ids.map((b) =>
            cyfs.PeopleId.try_from_object_id(cyfs.ObjectId.copy_from_slice(b)).unwrap()
        )
    };
}
