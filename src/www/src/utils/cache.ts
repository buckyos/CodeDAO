import { now } from 'lodash';
// import type {cacheDataStruct, cacheResponse} from '../../@types/index'
import _ from 'lodash';
import { responseUser } from '../common/types';

export function cacheSet(key: string, data: string | Object, ttl: number) {
    const value: cacheDataStruct = {
        data: JSON.stringify(data),
        ttl,
        date: now()
    };
    const itemStr: string = JSON.stringify(value);
    localStorage.setItem(key, itemStr);
}

export function cacheGet(key: string): cacheResponse {
    const resp = localStorage.getItem(key);
    if (!resp) {
        return {
            ok: false,
            data: ''
        };
    }

    // ttl
    const value: cacheDataStruct = JSON.parse(resp);
    if (now() > value.date + value.ttl * 1000) {
        console.log('cache key timeout ', key);
        return {
            ok: false,
            data: ''
        };
    }

    return {
        ok: true,
        data: value.data
    };
}

export function setUserNameCache(id: string, name: string) {
    const resp = cacheGet('user_list');
    if (resp.ok) {
        const oldData = JSON.parse(resp.data);
        const newData = {
            ...oldData,
            [id]: name
        };
        cacheSet('user_list', newData, 3600);
    } else {
        const data: { [key: string]: string } = { [id]: name };
        cacheSet('user_list', data, 3600);
    }
}

export function getUserNameCache(id: string) {
    const resp = cacheGet('user_list');
    if (resp.ok) {
        const data = JSON.parse(resp.data);

        if (data[id]) {
            return data[id];
        }
    }
    return '';
}

export function updateUserInfoCache(email: string) {
    const result: cacheResponse = cacheGet('user_info');
    if (result.ok) {
        const value = JSON.parse(result.data);
        value.email = email;
        cacheSet('user_info', value, 3600);
    }
}

// export function getUserNameById(id: string): cacheResponse {
//     const resp  = cacheGet('user/list')
//     if (!resp.ok) {
//         return resp
//     }
//
//     const data = JSON.parse(resp.data)
//     const list: responseUser[] = data.data
//     const item = _.find(list, {id: id})
//
//     if (!item) {
//         return {
//             ok: false,
//             data: "",
//         }
//     }
//
//     return {
//         ok: true,
//         data: item.name,
//     }
// }
