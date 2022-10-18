export * from './request';
export * from './util';
import * as cyfs from 'cyfs-sdk';
import dayjs from 'dayjs';

/**
 * fileid转cyfs的o链接，在cyfs浏览器内直接使用
 * @param peopleId
 * @param fileId
 * @returns
 */
export function fileIdToCyfsURL(peopleId: string, fileId: string): string {
    if (!fileId) return '';
    return `cyfs://o/${peopleId}/${fileId}`;
}

/**
 * 解析TextObject为文本
 * @param textObj
 * @returns
 */
export function parseTextObject(textObj: cyfs.TextObject) {
    const decoder = new cyfs.TextObjectDecoder();
    let param: string;
    const r = decoder.from_raw(textObj.encode_to_buf().unwrap());
    if (r.err) {
        const msg = `decode failed, ${r}.`;
        console.error(msg);
        return null;
    }
    return r.unwrap().value;
}

/**
 * 时间戳转本地日期
 * @param nS
 * @returns
 */
export function getLocalTime(nS: number) {
    const ms = nS / 1000;
    const today = new Date().toLocaleDateString();
    const parseDay = Date.parse(today) / 1000;
    const diffDay = Math.floor((parseDay - ms) / 60 / 60 / 24);
    // 当天内
    if (diffDay === -1) {
        return dayjs(nS).format('HH:mm');
    }
    // 昨天
    if (diffDay === 0) {
        return `Yesterday ${dayjs(nS).format('HH:mm')}`;
    }
    // 两天前
    if (diffDay > 0) {
        return dayjs(nS).format('YYYY-M-D HH:mm');
    }
}
