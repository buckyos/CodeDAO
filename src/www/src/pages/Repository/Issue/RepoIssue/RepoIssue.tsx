import React from 'react';
import { useHistory, useParams } from 'react-router-dom';
import { Spin, Skeleton } from 'antd';
import { RequestRepoIssue, ResponseRepositoryIssues, ResponseIssueItem } from '@src/types/rpc_def';
import { CommentOutlined } from '@ant-design/icons';
import { requestLocal, useRequestShadow } from '../../../../utils/index';
import { IssueNameDate } from '../RepoIssueDetail/RepoIssueDetail';
import styles from './RepoIssue.module.less';
import { useTranslation } from 'react-i18next';
import IssueOpenIcon from '@src/assets/images/issue_open.png';
import IssueCloseIcon from '@src/assets/images/issue_close.png';

const RepoIssue: React.FC = () => {
    const history = useHistory();
    const { owner, object_id } = useParams<RepoUrlParams>();
    const { t } = useTranslation();

    const { data, loading } = useRequestShadow<{data: ResponseRepositoryIssues}>(
        () => {
            const req: RequestRepoIssue = {
                owner: owner,
                id: object_id,
                author_name: owner,
                name: object_id
            };
            return requestLocal('repo/issues', req);
        },
        [],
        'fetch issue data failed'
    );

    const locationNewIssue = () => {
        history.push(`/${owner}/${object_id}/issues/new`);
    };

    const locationIssue = (issue: ResponseIssueItem) => {
        history.push(`/${owner}/${object_id}/issues/${issue.id}`);
    };

    return (
        <div className={styles.repoIssue}>
            <div className={styles.repoIssueAction}>
                <button className={styles.issueCreateBtn} onClick={locationNewIssue}>
                    {t('repository.file.issue.new')}
                </button>
            </div>
            {(loading || data == undefined) && <Skeleton ></Skeleton>}
            {!loading && data != undefined && 
            <div>
                <div className={styles.issueHeader}>
                    <div className={styles.issueNumber}>
                        <div>
                            <img src={IssueOpenIcon} alt="" />
                            <span> {data.data.open_count} open </span>
                        </div>
                        <div>
                            <img src={IssueCloseIcon} />
                            <span> {data.data.close_count} close </span>
                        </div>
                    </div>
                </div>
                {data.data.issues.map((issue: ResponseIssueItem, key) => {
                    let icon = issue.status == "open" ? <img src={IssueOpenIcon} alt="" />: <img src={IssueCloseIcon} alt="" />
                    return (
                        <div
                            className={styles.listItem}
                            key={key}
                            onClick={() => locationIssue(issue)}
                        >
                            <div className={styles.repoIssueStatus}>
                                <div>
                                    {icon}
                                </div>
                            </div>
                            <div className={styles.titleWrap}>
                                <div className={styles.listTitle}>
                                    <div>{issue.title}</div>
                                    {!!issue.topic_comment_length && (
                                        <div className={styles.commentLength}>
                                            <CommentOutlined className={styles.commentIcon} />
                                            <dd>{issue.topic_comment_length}</dd>
                                        </div>
                                    )}
                                </div>
                                <div className={styles.subtitle}>
                                    <IssueNameDate className={styles.listName} issue={issue} />
                                </div>
                            </div>
                        </div>
                    );
                })}
            </div>}
        </div>
    );
};

export default RepoIssue;
