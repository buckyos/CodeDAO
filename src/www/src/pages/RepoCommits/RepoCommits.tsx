import React, { useCallback, useEffect, useRef } from 'react';
import { RouteComponentProps, withRouter } from 'react-router-dom';
import { Modal, message } from 'antd';
import { commitModelAtom } from '@src/stores/commit';
import { observer } from 'mobx-react';
import { RequestCommits, ResponseCommit } from '@src/types';
import { toYMDHMS, requestLocal } from '@src/utils';

import { NftModal, NftIcon } from '@src/components/NftModal/Nft';
import { useSetRecoilState } from 'recoil';
import CommitPointIcon from '@src/assets/images/commit_point.png';
import HashLogoIcon from '@src/assets/images/hash_logo.png';
import FileIcon from '@src/assets/images/file.png';
import styles from './RepoCommits.css';

const RepoCommits: React.FC<RouteComponentProps> = ({ history, match }) => {
    const { owner, object_id, branch } = match.params as RepoUrlParams;
    const [commits, setCommits] = React.useState<ResponseCommit[]>([]);
    const setCommitModel = useSetRecoilState(commitModelAtom);

    useEffect(() => {
        (async function () {
            const req: RequestCommits = {
                owner: owner,
                id: object_id,
                author_name: owner,
                name: object_id,
                branch: branch
            };
            const r = await requestLocal<{ data: ResponseCommit[] }>('repo/commits', req);
            if (r.err) {
                message.error(`fetch dec app data err ${r.msg}`);
                return;
            }
            const data = r.data!.data;
            // const data = (r.data!.data || []).sort((a: ResponseCommit, b: ResponseCommit)=>{
            //     return JSON.parse(b.author).timestamp - JSON.parse(a.author).timestamp
            // })
            setCommitModel({
                commits: data
            });

            setCommits(data);
        })();
    }, []);

    const locationCommitDetail = (commitID: string) => {
        history.push(`/${owner}/${object_id}/commit/${commitID}`);
    };

    const copyCommitId = (commit: ResponseCommit) => {
        const aux = document.createElement('input');
        aux.setAttribute('value', commit.oid);
        document.body.appendChild(aux);
        aux.select();
        document.execCommand('copy');
        document.body.removeChild(aux);
    };

    return (
        <div className="file-content">
            <NftModal />

            <div>
                {commits.map((commit: ResponseCommit, index) => {
                    const authorName = JSON.parse(commit.author).name;
                    const committer = JSON.parse(commit.committer);
                    
                    const commitDate = committer.date.split(' ')[0] * 1000;

                    return (
                        <div className={styles.commitItem} key={index}>
                            <div className={styles.commitItemLine}>
                                <span className={styles.commitItemCircle}>
                                    <img src={CommitPointIcon} />
                                </span>
                            </div>
                            <div className={styles.commitItemTitle}>
                                Commit on {toYMDHMS(commitDate)}
                            </div>
                            <div className={styles.commitItemMain}>
                                <div className={styles.commitItemMainLeft}>
                                    <div className={styles.commitItemMessage}>{commit.message}</div>
                                    <div className={styles.commitItemInfo}>
                                        <span className={styles.circle}></span>
                                        <span className={styles.name}>{authorName}</span>
                                    </div>
                                </div>

                                <div className={styles.commitItemMainRight}>
                                    <NftIcon id={commit.id} />

                                    <div className={styles.commitItemHash}>
                                        <div className={styles.hashInfo}>
                                            <span
                                                className={styles.hashInfoCopy}
                                                onClick={() => copyCommitId(commit)}
                                            >
                                                <img src={HashLogoIcon} />
                                            </span>
                                            <span>{commit.oid.slice(0, 7)}</span>
                                        </div>
                                        <div
                                            className={styles.showBtn}
                                            onClick={() => {
                                                locationCommitDetail(commit.oid);
                                            }}
                                        >
                                            <img src={FileIcon} />
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    );
                })}
            </div>
        </div>
    );
};

export default withRouter(observer(RepoCommits));
