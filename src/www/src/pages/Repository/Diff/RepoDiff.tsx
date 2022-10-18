import React from 'react';
import { useParams } from 'react-router-dom';
import { requestLocal, useRequestShadow } from '../../../utils/index';
import { Skeleton } from 'antd';
import CommitDiff from '../../../components/CommitDiff/CommitDiff';
import dayjs from 'dayjs';
// @ts-ignore
import styles from './RepoDiff.css';

const RepoDiff: React.FC = () => {
    const { owner, object_id, hashId } = useParams<RepoDiffUrlParams>();
    const { data, loading } = useRequestShadow<{ data: ResponseRepositoryCommitDetail }>(
        () =>
            requestLocal('repo/commit', {
                owner: owner,
                id: object_id,
                commitId: hashId,
                author_name: owner,
                name: object_id,
                commit_id: hashId
            }),
        [],
        'fetch repo commit diff error'
    );

    if (loading || data == undefined) {
        return <Skeleton active />;
    }
    // const resp = data as ResponseCommitShowDiff
    // commit info
    // console.log("RepoDiff ", resp)
    const info = data.data.header_info;
    const author = JSON.parse(info.author);
    const date = dayjs.unix(author.date.split(" ")[0]).format('YYYY-MM-DD');

    return (
        <div className={styles.main}>
            <div className={styles.header}>
                <div className={styles.message}>{info.message}</div>
                <div className={styles.commit}>
                    <div></div>
                    <div className={styles.name}>{author.name}</div>
                    <div>committed on {date}</div>
                    <div className={styles.hash}>commit {info.oid}</div>
                </div>
            </div>

            <CommitDiff diffData={data.data.diff} />
        </div>
    );
};

export default RepoDiff;
