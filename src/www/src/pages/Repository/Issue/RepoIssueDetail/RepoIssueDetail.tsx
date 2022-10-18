import React, { useCallback, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import { message, Spin } from 'antd';
import { ResponseIssueItem, ResponseIssue, RequestIssueDetail } from '@src/types';
import { classSet, requestLocal, useRequestShadow } from '../../../../utils';
import UserLogo from '../../../../components/UserLogo/UserLogo';
import styles from './RepoIssueDetail.module.less';
import Editor from '../../../../components/Editor/Editor';
import { stackInfo } from '@src/utils/stack';
import { NftModal, NftIcon } from '../../../../components/NftModal/Nft';

import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import { MdShow } from '../../../../components/MdShow/MdShow';
dayjs.extend(relativeTime);
import { useTranslation } from 'react-i18next';

export const IssueNameDate: React.FC<{
    issue: ResponseIssueItem;
    className?: string;
    showNft?: boolean;
}> = ({ issue, className, showNft }) => {
    const fromNow = dayjs(issue.date).fromNow();

    return (
        <div className={className ? className : styles.itemHeader}>
            <div className={styles.itemUser}>{issue.user_name}</div>
            <div>commented on {fromNow}</div>

            {showNft && (
                <div className={styles.nftIcon}>
                    <NftIcon id={issue.object_id} />
                </div>
            )}
        </div>
    );
};

export const IssueComment: React.FC<{ issue: ResponseIssueItem; className?: string }> = ({ issue }) => {
    return (
        <div className={styles.wrap}>
            <UserLogo className={styles.logo} user_id={issue.user_id} />
            <div className={styles.quota}>
                <IssueNameDate issue={issue} showNft={true} />
                <div className={styles.issueBlock}>
                    <MdShow content={issue.content} />
                </div>
                {/* <ReactMarkdown className={styles.issueBlock} children={issue.content} remarkPlugins={[remarkGfm]} /> */}
            </div>
        </div>
    );
};

const RepoIssueDetail: React.FC = () => {
    const { object_id, issue_id, owner } = useParams<RepoIssueUrlParams>();
    const { t } = useTranslation();

    const [content, setContent] = React.useState('');
    const req: RequestIssueDetail = {
        issue_id: issue_id,
        id: object_id,
        owner: owner,
        author_name: owner,
        name: object_id
    };
    const { data, loading, request } = useRequestShadow<{ data: ResponseIssue }>(
        () => requestLocal('repo/issue', req),
        [issue_id],
        'get repo issue data failed'
    );

    const onSubmit = useCallback(() => {
        if (content == '') {
            message.error(t('error.issue.content.empty'));
            return;
        }
        const req: RequestRepoIssueComment = {
            issue_id: issue_id,
            author_name: owner,
            name: object_id,
            content: content,
            user_id: stackInfo.owner.toString()
        };
        requestLocal('repo/issue/comment', req).then((resp) => {
            if (resp.err) {
                message.error(resp.msg);
                return;
            }

            //  刷新 comment列表
            request();
        });
    }, [content]);


    const onCloseIssue = useCallback(() => {
        const req: RequestRepoIssueClose = {
            name: object_id,
            issue_id: issue_id,
            author_name: owner,
        };
        requestLocal('repo/issue/close', req).then((resp) => {
            if (resp.err) {
                message.error(resp.msg);
                return;
            }
        });
    }, []);


    if (loading || data == undefined) {
        return <Spin />;
    }
    const topic = data.data.topic;
    const issues = data.data.issues;

    // console.log(topic);

    return (
        <div className={styles.repoIssue}>
            <NftModal />

            <div className={styles.title}>
                <h1>
                    {topic.title}
                    <span>#{topic.id}</span>
                </h1>
            </div>
            <div className={styles.status}>
                {topic.status == "open" ? <div className={styles.tag}>{topic.status}</div> : <div className={styles.closeTag}>{topic.status}</div>}
                <IssueNameDate className={styles.statusName} issue={topic} />
            </div>

            <div className={styles.view}>
                <div className={styles.left}>
                    {/* <IssueComment issue={issue} />*/}
                    {issues?.map((item: ResponseIssueItem, key: number) => {
                        return <IssueComment issue={item} key={key} />;
                    })}

                    <div className={styles.wrap}>
                        <UserLogo className={styles.logo} user_id={stackInfo.owner.toString()} />
                        <div className={styles.quota}>
                            <div className={styles.empty}></div>
                            <Editor onChange={setContent} />

                            <div className={styles.submit}>
                                {topic.status == "open" && 
                                    <button className={classSet([styles.button, styles.closeButton])} onClick={onCloseIssue}>
                                        Close issue
                                    </button>
                                }

                                <button className={styles.button} onClick={onSubmit}>
                                    Comment
                                </button>
                            </div>

                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default RepoIssueDetail;
