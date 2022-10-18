import React, { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { getBranch, requestLocal, requestTarget, useRequestShadow } from '../../../utils';
import { languageColor } from '../../../utils/color';
import { useRecoilState } from 'recoil';
import { repositoryCurrentBranchAtom } from '../../../stores/repository';
import style from './RepoLanguage.css';

export const RepoLanguage: React.FC = () => {
    const { owner, object_id } = useParams<{ owner: string; object_id: string; branch?: string }>();
    const branch = getBranch();

    const req = {
        branch: branch,
        author_name: owner,
        name: object_id
    };
    const { data, loading } = useRequestShadow<{ data: { [key: string]: number } }>(
        () => requestLocal('repo/analysis/language', req),
        [],
        ''
    );

    const disposeLanguageData = (data: { [key: string]: number }): LanguageData[] => {
        const arr: LanguageTypeData[] = Object.entries(data).map(([key, val]) => ({
            language_type: key,
            code: val
        }));
        console.log('disposeLanguageData', arr);

        const total = arr.reduce((total, item) => {
            total += item.code;
            return total;
        }, 0);
        console.log('disposeLanguageData totle', total);

        // let count = 0;
        // const newData = data.filter(ff => ff.type.toLocaleLowerCase() !== 'total' && ff.code !== 0 )
        // newData.forEach(ff => count += ff.code)
        // if(count === 0){
        //     return []
        // }
        // return
        return arr
            .map((ff) => {
                const color = languageColor[ff.language_type] ?
                    languageColor[ff.language_type].color || '#D2DE5E' :
                    '#D2DE5E';
                const percent = Number((Math.round((ff.code / total) * 1000) / 1000).toFixed(3)); // 保留3位小数
                return {
                    ...ff,
                    color,
                    percent
                };
            })
            .sort((a, b) => b.percent - a.percent);
    };

    if (loading) {
        return <div></div>;
    }

    const languageData = disposeLanguageData(data!.data || {});

    if (languageData.length === 0) {
        return <div></div>;
    }

    return (
        <div className={style.language}>
            <div className={style.languageTitle}>Languages</div>
            <div className={style.languageBox}>
                {languageData
                    .filter((ff) => ff.percent >= 0.05)
                    .map((ff: LanguageData, i) => {
                        return (
                            <div
                                className={style.languageItem}
                                key={i}
                                style={{
                                    backgroundColor: ff.color,
                                    width: `${(ff.percent * 100).toFixed(1)}%`
                                }}
                            ></div>
                        );
                    })}
            </div>
            <div className={style.languageList}>
                {languageData
                    .filter((ff) => ff.percent >= 0.05)
                    .map((ff, i) => (
                        <div className={style.languageListItem} key={i}>
                            <div
                                className={style.languageColor}
                                style={{ backgroundColor: ff.color }}
                            ></div>
                            <div className={style.languageType}>{ff.language_type}</div>
                            <div className={style.languagePercent}>{`${(ff.percent * 100).toFixed(
                                1
                            )}%`}</div>
                        </div>
                    ))}
            </div>
        </div>
    );
};
