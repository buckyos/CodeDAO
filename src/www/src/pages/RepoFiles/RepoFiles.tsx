import React, { useCallback } from 'react';
import { useHistory, useParams, withRouter } from 'react-router-dom';
import {
    repositoryChildPath,
    repositoryCurrentBranchAtom,
    repositoryLastCommitAtom,
    useCleanSwitchPath
} from '@src/stores/repository';
import { enCodePath, toYMDHMS } from '@src/utils';
import { MdShow } from '@src/components/MdShow/MdShow';
import { useRecoilState } from 'recoil';
import DirLogoIcon from '@src/assets/images/dir_logo.png';
import FileLogoIcon from '@src/assets/images/file_logo.png';
import styles from './RepoFiles.css';

// RepoFiles
// 仓库文件列表
const RepoFiles: React.FC<{ data: RepoFileList }> = ({ data }) => {
    const { object_id, owner } = useParams<RepoUrlParams>();
    const history = useHistory();
    const [childPath] = useRecoilState(repositoryChildPath);
    const [branch] = useRecoilState(repositoryCurrentBranchAtom);
    const [lastCommit] = useRecoilState(repositoryLastCommitAtom);
    const cleanRepositoryPathData = useCleanSwitchPath();
    const { files, readme } = data;

    const locationFile = useCallback(
        (item: ResponseRepoFile) => {
            const val = item.file;
            const fileFullPath: string = childPath != '' ? childPath + '/' + val : val;
            const fileType = item.file_type;
            // const fileType = item.fileType == 'file' ? 'blob': 'tree'
            const path = `/${owner}/${object_id}/${fileType}/${branch}/${enCodePath(fileFullPath)}`;
            cleanRepositoryPathData();

            history.push(path);
        },
        [object_id, owner]
    );

    // 返回上一级目录
    const locationBack = () => {
        const filePathArr = childPath.split('/');
        filePathArr.pop();
        const fileFullPath: string = filePathArr.join('/');

        history.push(`/${owner}/${object_id}/tree/${branch}/${fileFullPath}`);
    };

    const locationCommitDiff = (commitId: string) => {
        history.push(`/${owner}/${object_id}/commit/${commitId}`);
    };

    return (
        <div>
            <h4 className={styles.repoHeader}>
                <div className={styles.repoHeaderLeft}>
                    <span>{lastCommit.author}</span>
                    <span>{lastCommit.message}</span>
                </div>
                <div className={styles.repoHeaderRight}>
                    <span>{lastCommit.commit}</span>
                    <span>{toYMDHMS(lastCommit.date)}</span>
                </div>
            </h4>
            <div className={styles.codeContent}>
                {childPath != '' && (
                    <div className={styles.fileItem}>
                        <div className={styles.name} onClick={locationBack}>
                            <img className={styles.dirLogo} src={DirLogoIcon} />
                            ..
                        </div>
                    </div>
                )}

                {files.map((item: ResponseRepoFile, key) => {
                    // const date = JSON.parse(item.date).date.split(' ')[0] * 1000;
                    const date = item.date;
                    // console.log(date, item.file);

                    return (
                        <div key={key} className={styles.fileItem}>
                            <div className={styles.name} onClick={() => locationFile(item)}>
                                {item.file_type === 'tree' ? (
                                    <img className={styles.dirLogo} src={DirLogoIcon} />
                                ) : (
                                    <img className={styles.fileLogo} src={FileLogoIcon} />
                                )}
                                {item.file}
                            </div>
                            <div className={styles.commitInfo}>
                                <span
                                    className={styles.commit}
                                    onClick={() => locationCommitDiff(item.commit)}
                                >
                                    {item.commit.substring(0, 8)}
                                </span>
                                <span className={styles.message} title={item.message}>{item.message}</span>
                            </div>
                            <div className={styles.date}>{toYMDHMS(date)}</div>
                        </div>
                    );
                })}
                {readme!.type && (
                    <div className={styles.repoMd}>
                        <div className={styles.repoMdHeader}>
                            <span className={styles.repoMdRound}></span>
                            <span>README.md</span>
                        </div>
                        <div className={styles.repoMdContent}>
                            <MdShow content={readme!.content} />
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

export default RepoFiles;
