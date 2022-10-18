// ood base root. this is  for  repo rebuild dir
import * as path from 'path';
import { bucky_time_2_js_date } from 'cyfs-sdk';

export function splitLines(source: string): string[] {
    const lines = source.split(/[\r\n]+/);
    return lines;
}

/* eslint-disable */
export function getDateByObjectTime(source: any): number {
    return bucky_time_2_js_date(source).getTime();
}

export function dealPage<s>(
    data: Array<s>,
    pageIndex: number,
    pageSize: number
): Array<s> {
    const from = pageIndex * pageSize;
    return data.slice(from, from + pageSize);
}
