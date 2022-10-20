import React, { useEffect, Component, useState } from 'react';
import { useParams } from 'react-router-dom';
import { message } from 'antd';
import { requestLocal, toYMDHMS } from '@src/utils';
import CommitsGraph from '@src/components/Graph/CommitGraph';
import styles from './RepoGroph.css';

interface CommitInfo {
    hash: string;
    sha: string;
    head: string[];
    message: string;
    parents: string[];
    timestamp: string;
    // index: number,   // circle 所在列
}

const RepoGraph: React.FC = () => {
    const { owner, object_id, branch: urlBranch } = useParams<RepoUrlParams>();
    const [commits, setCommits] = useState<CommitInfo[]>([]);
    const [selected, setselected] = useState<string>('');

    useEffect(() => {
        ininCommitInfo();
    }, [owner, object_id, urlBranch]);

    const ininCommitInfo = async () => {
        const branch = 'master';
        const r = await requestLocal<{ data: CommitInfo[] }>('repo/log/graph', {
            author_name: owner,
            name: object_id,
            branch: branch
        });
        if (r.err || r.data === undefined) {
            message.error(`fetch dec app data err ${r.msg}`);
            return;
        }

        const commits = r.data.data.map((item) => {
            return {
                ...item,
                sha: item.hash
            };
        });
        console.log(commits);
        setCommits(commits);
    };

    function handleClick(sha: string) {
        console.log('graph:', sha, 'on click');
        setselected(sha);
    }

    return (
        <div style={{ display: 'flex' }}>
            <CommitsGraph commits={commits} onClick={handleClick} selected={selected} />
            <div className={styles.hashList}>
                {commits.map((commit) => {
                    return (
                        <div
                            key={commit.hash}
                            style={{ display: 'flex', alignItems: 'center', height: 40 }}
                        >
                            {commit.head.length > 0 &&
                                commit.head.map((head, key) => (
                                    <div
                                        key={key}
                                        style={{
                                            marginRight: 5,
                                            border: '1px solid #0085d9',
                                            fontSize: 10,
                                            padding: '1px 3px',
                                            borderRadius: 8
                                        }}
                                    >
                                        {head}
                                    </div>
                                ))}
                            <div style={{ lineHeight: '20px', flex: 1 }}>{commit.message}</div>
                            <div style={{ lineHeight: '20px' }}>
                                {toYMDHMS(parseInt(commit.timestamp) * 1000)}
                            </div>
                            <div style={{ lineHeight: '20px', marginLeft: 20 }}>{commit.hash}</div>
                        </div>
                    );
                })}
            </div>
        </div>
    );
};

export default RepoGraph;
