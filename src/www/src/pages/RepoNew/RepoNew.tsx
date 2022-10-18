import React, { useCallback } from 'react';
import { useHistory } from 'react-router-dom';
import { message, Select, Radio, Spin } from 'antd';
import _ from 'lodash';
import { requestLocal, classSet } from '../../utils/index';
import { validateName } from '../../utils/validate/validate';
import { useRecoilState } from 'recoil';
import { userInfoAtom } from '../../stores/user';
import { useTranslation } from 'react-i18next';
import { AuthorType } from '@src/types';
import { notifyCreateRepository } from '@src/apis/pubsub';
import styles from './RepoNew.module.less';

const { Option } = Select;

const RepoNew: React.FC = () => {
    const history = useHistory();
    const [options, setOptions] = React.useState<ResponseOptionsAuthor[]>([]);
    const [repoName, setRepoName] = React.useState('');
    const [author, setAuthor] = React.useState<ResponseOptionsAuthor>({
        id: '',
        type: AuthorType.User,
        name: ''
    });
    // const [authorType, setAuthorType] = React.useState('')
    const [repoDesc, setRepoDesc] = React.useState('');
    const [isPrivate, setIsPrivate] = React.useState(1);
    const [userInfo] = useRecoilState(userInfoAtom);
    const { t } = useTranslation();

    React.useEffect(() => {
        (async function () {
            const req: RequestOnlyOwnerID = { owner: userInfo.id };
            const r = await requestLocal<{ data: ResponseOptionsAuthor[] }>('authors', req);
            if (r.err || r.data === undefined) {
                console.error('author/list failed', r);
                message.error(t('error.repository.new.options.fail'));
                return;
            }
            setOptions(r.data.data);
            const defaultValue = r.data.data[0];
            setAuthor(defaultValue);
        })();
    }, []);

    const onSubmit = async () => {
        if (repoName == '') {
            message.error(t('error.repository.new.name.empty'));
            return;
        }

        if (!_.isEmpty(repoName) && !validateName(repoName)) {
            message.error(
                `${t('error.repository.new.name')} [${repoName}] ${t(
                    'error.repository.new.format'
                )}`
            );
            return;
        }

        if (author.id == '') {
            // user or org
            message.error(t('error.repository.new.owner.empty'));
            return;
        }

        const req: RequestRepositoryNew = {
            // owner: stackInfo.owner.toString(),
            author_id: author.id,
            name: repoName,
            description: repoDesc,
            is_private: isPrivate,
            author_type: author.type,
            author_name: author.name
        };
        const r = await requestLocal('repo/new', req);
        if (r.err) {
            console.error('create repo new failed', r);
            message.error(`${t('error.repository.new.fail')}: ${r.msg}`);
            return;
        }

        message.success(t('success.repository.new'));
        // 通知发布订阅中心
        await notifyCreateRepository(repoName);
        setTimeout(() => {
            history.push('/');
        }, 1500);
    };

    const onSelectAuthor = useCallback(
        (val: string) => {
            const target = options.find((option) => {
                return option.id == val;
            });
            if (target) {
                setAuthor(target);
            }
        },
        [options]
    );

    const renderOptions = () => {
        return (
            <div className={styles.ownerReponame}>
                <label className="require">{t('create.repository.owner')}</label>
                <div>
                    <Select
                        className={styles.ownerReponameSelect}
                        value={author.id}
                        onChange={onSelectAuthor}
                        loading={_.isEmpty(options)}
                    >
                        {options?.map((item: ResponseOptionsAuthor) => {
                            return (
                                <Option key={item.name + item.id} value={item.id} title={item.name}>
                                    {item.name}
                                </Option>
                            );
                        })}
                    </Select>
                </div>
            </div>
        );
    };

    return (
        <div>
            <div className={styles.newRepoContent}>
                <div className={styles.newRepoTitle}>{t('create.repository.title')}</div>
                {renderOptions()}

                <div className={styles.ownerReponame}>
                    <label className={styles.require}>{t('create.repository.name')}</label>
                    <input
                        className={styles.ownerReponameInput}
                        value={repoName}
                        onChange={(e) => setRepoName(e.target.value)}
                    />
                </div>
                <div className={styles.ownerReponameHelp}>{t('create.repository.name.remark')}</div>
                <div className={styles.ownerReponame}>
                    <label> {t('create.repository.visibility')} </label>
                    <Radio.Group
                        defaultValue="1"
                        buttonStyle="solid"
                        onChange={(e) => setIsPrivate(Number(e.target.value))}
                    >
                        <Radio value="1">{t('create.repository.private')}</Radio>
                        <Radio value="0">{t('create.repository.public')}</Radio>
                    </Radio.Group>
                </div>

                <div className={styles.ownerReponameReponameDesc}>
                    <label>{t('create.repository.description')}</label>
                    <textarea
                        className={styles.ownerDescInput}
                        rows={3}
                        value={repoDesc}
                        onChange={(e) => setRepoDesc(e.target.value)}
                    />
                </div>
                <div className={styles.ownerReponameHelp}>
                    {t('create.repository.description.remark')}
                </div>

                <div className={styles.createRepoButtons}>
                    <div className={classSet([styles.createRepoButton, styles.createRepoButtonCancel])} onClick={onSubmit}>
                        {t('create.repository.new')}
                    </div>
                    <div
                        className={styles.createRepoButtonCancel}
                        onClick={() => {
                            history.push('/');
                        }}
                    >
                        {t('create.repository.cancel')}
                    </div>
                </div>
            </div>
        </div>
    );
};

export default RepoNew;
