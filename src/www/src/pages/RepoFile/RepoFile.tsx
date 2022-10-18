import React from 'react';
import { useParams } from 'react-router-dom';
import { toYMDHMS, requestTarget, useRequestShadow, requestLocal } from '../../utils/index';
import { Spin } from 'antd';
import { requestFileData } from '../../types';
import { useTranslation } from 'react-i18next';
import styles from './RepoFile.css';

// useRepoFile
// typeWord, [file: blob, dir: tree]
export function useRepoFile(req: requestFileData, typeWord: string, branch: string) {
    const fileFullPath = (function getUrlFileFullPath(
        owner: string,
        object_id: string,
        branch: string
    ) {
        const reg = new RegExp(`^#/${owner}/${object_id}/?(src/${branch})?$`);
        if (location.hash.match(reg)) {
            return '';
        }
        console.log(owner, object_id);

        const word = location.hash.replace(`#/${owner}/${object_id}/${typeWord}/${branch}/`, '');
        // 处理中文名字的情况
        return decodeURIComponent(word);
    })(req.author_name, req.name, branch);

    req.file_name = fileFullPath;
    req.path = fileFullPath;

    const { data, loading, request } = useRequestShadow<ResponseFile>(
        () => requestLocal('repo/file', req),
        [fileFullPath],
        'get repo file data failed'
    );

    return { data, loading, request, fileFullPath };
}

const RepoFile: React.FC = () => {
    const { owner, object_id, branch } = useParams<RepoUrlParams>();
    const { data, loading, fileFullPath } = useRepoFile(
        {
            name: object_id,
            author_name: owner,
            file_name: '',
            hash: '',
            branch: branch
        },
        'blob',
        branch
    );
    const { t } = useTranslation();

    if (loading || data === undefined || data.fileData === undefined) {
        return <Spin />;
    }

    const fileData = data.fileData;
    const info = fileData.info;

    const noShow = fileData.bigFile || fileData.notSupport;

    return (
        <div className={styles.fileContent}>
            <div className={styles.fileContentHeader}>
                <div>
                    {info.author} {info.message}
                </div>
                <div>
                    Latest commit {info.commit.substring(0, 8)} on {toYMDHMS(info.date)}
                </div>
            </div>
            <div className={styles.fileContentMain}>
                {!noShow && (
                    <div className={styles.fileContentMainHeader}>
                        {fileData.content.length} lines | {info.fileSize} Bytes | {fileFullPath}
                    </div>
                )}
                <div className={styles.fileContentList}>
                    {fileData.bigFile && <div>{t('repository.file.display.too_big')}</div>}
                    {fileData.notSupport && (
                        <div>{t('repository.file.display.not_suport_type')}</div>
                    )}
                    {!noShow && (
                        <table>
                            <tbody>
                                {fileData.content.map((ff) => (
                                    <tr className={styles.fileContentItem} key={ff.line}>
                                        <td className={styles.fileContentLeft}>{ff.line}</td>
                                        <td className={styles.fileContentRight}>{ff.content}</td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    )}
                </div>
            </div>
        </div>
    );
};

export default RepoFile;
