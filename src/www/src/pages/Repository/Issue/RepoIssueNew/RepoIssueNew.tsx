import React from 'react';
import { useHistory, useParams } from 'react-router-dom';
import { message } from 'antd';
import { useRecoilState } from 'recoil';
import { useTranslation } from 'react-i18next';
import { requestLocal } from '../../../../utils/index';
import { stackInfo } from '@src/utils/stack';
import UserLogo from '../../../../components/UserLogo/UserLogo';
import Editor from '../../../../components/Editor/Editor';
import { classSet } from '../../../../utils/util';
import styles from './RepoIssueNew.module.less';
import { userInfoAtom } from '../../../../stores/user';

const RepoIssueNew: React.FC = () => {
    const { owner, object_id } = useParams<RepoUrlParams>();
    const history = useHistory();
    const [userInfo] = useRecoilState(userInfoAtom);
    const [title, setTitle] = React.useState('');
    const [content, setContent] = React.useState('');
    const { t } = useTranslation();

    const onSubmit = async () => {
        if (title == '') {
            message.error(t('error.issue.new.title.empty'));
            return;
        }
        if (content == '') {
            message.error(t('error.issue.new.content.empty'));
            return;
        }
        const req: RequestIssueCreate = {
            owner: owner,
            id: object_id,
            author_name: owner,
            name: object_id,
            title: title,
            content: content,
            user_id: stackInfo.owner.toString()
        };
        const r = await requestLocal('issue/new', req);
        if (r.err) {
            console.log('create issue data failed');
            return;
        }

        message.success(t('success.issue.new'));
        setTimeout(() => {
            history.push(`/${owner}/${object_id}/issues`);
        }, 1500);
    };

    return (
        <div className={styles.repoIssue}>
            <div className={styles.wrap}>
                <div className={styles.logoBox}>
                    <UserLogo className={styles.logo} user_id={userInfo.id} />
                </div>

                <div className={styles.quota}>
                    <div className={styles.issueBlock}>
                        <input
                            className={styles.issueNewInput}
                            placeholder="issue tittle"
                            onChange={(e) => setTitle(e.target.value)}
                            type="text"
                        />
                    </div>
                    <Editor onChange={setContent} />
                    <div className={styles.submit}>
                        <button className={styles.button} onClick={onSubmit}>
                            {t('repository.file.issue.submit')}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default RepoIssueNew;
