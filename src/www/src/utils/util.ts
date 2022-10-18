import React, { DependencyList, useEffect, useLayoutEffect, useState } from 'react';
import { message } from 'antd';
// import type {classKey} from '../../@types'
import dayjs from 'dayjs';
import { useParams } from 'react-router-dom';
import _ from 'lodash';
import { BuckyErrorCode } from 'cyfs-sdk';

// 简单的封装一下 useEffect + 请求数据处理
export const useRequest = (fn: Function, dependencies: DependencyList, error_message: string) => {
    const [data, setData] = useState([]);
    const [count, setCount] = useState(0);
    const [loading, setLoading] = useState(true);

    // 请求的方法 这个方法会自动管理loading
    const request = () => {
        setLoading(true);
        fn()
            /* eslint-disable */
            .then((r:any) => {
                if (r.err) {
                    message.error(error_message + r.msg);
                    return;
                }
                setData(r.data.data);
                setCount(r.data.count);
                // console.log('useRequest', r)
            })
            .finally(() => {
                setLoading(false);
            });
    };

    // 根据传入的依赖项来执行请求
    useEffect(() => {
        request();
    }, dependencies);

    return {
        // 请求获取的数据
        data,
        count,
        // loading状态
        loading,
        // 请求的方法封装
        request
    };
};

// 简单的封装一下 useEffect + 请求数据处理
export const useRequestShadow = <S>(fn: Function, dependencies: DependencyList, error_message: string) => {
    const [data, setData] = useState<S>();
    const [loading, setLoading] = useState(true);

    // 请求的方法 这个方法会自动管理loading
    const request = () => {
        setLoading(true);
        fn()
            /* eslint-disable */
            .then((r:any) => {
                if (r.err) {
                    if(r.m_code !== BuckyErrorCode.Timeout){ // 超时暂不提示错误
                        message.error(error_message);
                    }
                    return;
                }
                setData(r.data);
                // console.log('useRequest', r)
            })
            .finally(() => {
                setLoading(false);
            });
    };

    // 根据传入的依赖项来执行请求
    useEffect(() => {
        request();
    }, dependencies);

    return {
        // 请求获取的数据
        data,
        // loading状态
        loading,
        // 请求的方法封装
        request
    };
};

export function classSet(classes: classKey[]): string {
    return classes.map((item: classKey):string => {
        if (typeof item === 'string') {
            return item;
        } else {
            for (const key in item) {
                if (item[key]) {
                    return key;
                } else {
                    return '';
                }
            }
            return '';
        }
    }).filter((item) => item).join(' ');
}

export function toYMDHMS(time: number | string) {
    return dayjs(time).format('YYYY-MM-DD HH:mm:ss');
}

export function enCodePath(path: string): string { // 对特殊字符encode，含%的不允许创建
    let str = path;
    const obj: { [key: string]: (name: string) => string } = {
        '?': (name) => name.replace(/\?/g, encodeURIComponent('?')),
        '=': (name) => name.replace(/[=]/g, encodeURIComponent('=')),
        '#': (name) => name.replace(/#/g, encodeURIComponent('#')),
        '&': (name) => name.replace(/&/g, encodeURIComponent('&')),
        '/': (name) => name.replace(/\//g, encodeURIComponent('/')),
        '+': (name) => name.replace(/\+/g, encodeURIComponent('+'))
    };    
    for (const ff of Object.keys(obj)) {
        if(str.includes(ff)){
            str = obj[ff](str);
        }
    }
    return str;
}

export function checkName(name: string): boolean {
    return !(name.indexOf('%') > -1);
}

export function getBranch() {
    const { branch } = useParams<RepoUrlParams>();
    return _.isEmpty(branch) ? 'master' : branch;
}

export function getPageIndex() {
    const searchParams = new URLSearchParams(location.hash.split('?').pop());
    const index = Number(searchParams.get('pageIndex')) || 0;    
    return index;
}