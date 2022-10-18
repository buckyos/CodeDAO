import { Validator } from 'jsonschema';
// const { schema } = require("./ts_schema.js");
import { schema } from './ts_schema';

interface SchemaType {
    '$schema': string,
    'definitions': {
        /* eslint-disable */
        [key: string]: any
    }
}

export enum ErrorCode {
    RESULT_OK = 0,
    RESULT_INVALID_PARAM = 1
};

export interface IResult<T> {
    code: number;
    message?: string;
    data?: T
}

export type RPCResponse<T> = Promise<[IResult<T>]>;

const newSchema: SchemaType = schema;

/* eslint-disable */
function travel(path: string, def: any, val: any): { code: ErrorCode, message?: string } {
    if (val == null) {
        return { code: 0 };
    }
    else if (def.type === 'number' && val) {
        const v = val.toString().split('.');
        if (v.length > 1) {
            if (v[1].length > 2) {
                console.log(path, val);
                return { code: ErrorCode.RESULT_INVALID_PARAM, message: `${path} ${val} 最多两位小数` };
            }
        }
    } else if (def.type === 'object') {
        for (const v in def.properties) {
            if (def.properties[v]) {
                const r = travel(`${path}/${v}`, def.properties[v], val[v]);
                if (r.code !== 0) {
                    return r;
                }
            }
        }
    } else if (def.type === 'array') {
        for (let i = 0; i < val.length; i++) {
            const r = travel(`${path}/[${i}]`, def.items, val[i]);
            if (r.code !== 0) {
                return r;
            }
        }
    }

    if (def.$ref) {
        const defPath = def.$ref.split('/');
        const name = defPath[defPath.length - 1];
        const r = checkNumber(path, name, val);
        if (r.code !== 0) {
            return r;
        }
    }
    return { code: 0 };
}

/* eslint-disable */
function checkNumber(path: string, name: string, val: any): { code: number, message?: string } {
    const s = newSchema.definitions[name];

    return travel(path, s, val);
}

/* eslint-disable */
function convertNumberTravel(path: string, def: any, val: any, isMul: boolean) {
    if (def.type === 'object') {
        for (const v in def.properties) {
            if (def.properties[v]) {
                if (def.properties[v].type === 'number') {
                    isMul ? val[v] *= 100 : val[v] /= 100;
                } else {
                    convertNumberTravel(`${path}/${v}`, def.properties[v], val[v], isMul);
                }
            }
        }
    } else if (def.type === 'array') {
        for (let i = 0; i < val.length; i++) {
            if (def.items.type === 'number') {
                isMul ? val[i] *= 100 : val[i] /= 100;
            } else {
                convertNumberTravel(`${path}/[${i}]`, def.items, val[i], isMul);
            }
        }
    }

    if (def.$ref) {
        const defPath = def.$ref.split('/');
        const name = defPath[defPath.length - 1];
        convertNumberImpl(path, name, val, isMul);
    }
}

function convertNumberImpl(path: string, name: string, val: any, isMul: boolean) {
    const s = newSchema.definitions[name];

    convertNumberTravel(path, s, val, true);
}

// 遍历typename,如果是number类型,x100
// 一般用于参数处理
/* eslint-disable */
export function floatToInt(typename: string, val: any) {
    convertNumberImpl(typename, typename, val, true);
}

// 遍历typename,如果是number类型,/100
// 一般用于返回值处理
/* eslint-disable */
export function intToFloat(typename: string, val: any) {
    convertNumberImpl(typename, typename, val, false);
}

/* eslint-disable */
export async function argCheck(name: string, arg: any): RPCResponse<null> {
    console.log('newSchema---', JSON.stringify(newSchema));
    const v = new Validator();
    v.addSchema(newSchema, '/api');
    const result = v.validate(arg, {
        $ref: 'api#/definitions/' + name
    }, {
        allowUnknownAttributes: true
    });
    console.log('result---', JSON.stringify(result));
    if (result.errors.length > 0) {
        const result2 = result.errors.map((x) => {
            return JSON.stringify({
                property: x.property,
                message: x.message,
                instance: x.instance,
                name: x.name,
                argument: x.argument
            });
        }).join('\n');
        return [{ code: ErrorCode.RESULT_INVALID_PARAM, message: name + ' ' + result2 }];
    }

    // check number is int
    const r = checkNumber(name, name, arg);
    return [r];
}

// 数组长度有效性判定
/* eslint-disable */
export async function argArrayMinLength(name: string, arr: any[], minLength: number): RPCResponse<null> {
    if (arr.length < minLength) {
        return [{ code: ErrorCode.RESULT_INVALID_PARAM, message: `参数${name}长度:${arr.length}, 最小长度要求:${minLength}` }];
    }
    return [{ code: 0 }];
}

// 字符串不为空
export async function argStringIsNotEmpty(name: string, s: string): RPCResponse<null> {
    if (s.length === 0) {
        return [{ code: ErrorCode.RESULT_INVALID_PARAM, message: `参数${name}为空字符串` }];
    }
    return [{ code: 0 }];
}

// number 最小值判断
export async function argNumberMin(name: string, n: number, min: number): RPCResponse<null> {
    if (n <= min) {
        return [{ code: ErrorCode.RESULT_INVALID_PARAM, message: `参数${name}:${n}, min: ${min}` }];
    }
    return [{ code: 0 }];
}