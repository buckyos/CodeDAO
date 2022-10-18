import { DownOutlined } from '@ant-design/icons';
import { Dropdown, Menu, message, Spin } from 'antd';
import React, { useRef, useState } from 'react';
import { RouteComponentProps, useParams, withRouter } from 'react-router-dom';
import { stackInfo } from '@src/utils/stack';
import PromptDialog from '../../components/PromptDialog';
import { branchesAtom } from '../../stores/repository';
import { checkName, requestTarget } from '../../utils';
import { useRecoilState } from 'recoil';
import { useTranslation } from 'react-i18next';
import ForkIcon from '@src/assets/images/fork.png';
import styles from './RepoReleaseNew.module.less';

const RepoReleaseNew: React.FC<RouteComponentProps> = ({ history }) => {
    const { t } = useTranslation();
    const { owner, object_id, tag_name } = useParams<RepoUrlParams>();
    const [pageTitle, setPageTile] = React.useState(
        t('repository.file.release.new.headline') as string
    );
    const [tagName, setTagName] = React.useState('');
    const [title, setTitle] = React.useState('');
    const [content, setContent] = React.useState('');
    const [branch, setBranch] = React.useState('master');
    const [id, setId] = React.useState('');
    const [isEdit, setIsEdit] = React.useState(false);
    const childCmpRef = useRef<PromptDialogRefType>(null);
    const [deleteParam, setParam] = useState<DeleteParam>({ title: '', content: '' });
    const [branches] = useRecoilState(branchesAtom);

    React.useEffect(() => {
        if (tag_name) {
            setIsEdit(true);
            setPageTile(t('repository.file.release.edit.headline'));
            (async function () {
                await fetchData();
            })();
        }
    }, []);

    const fetchData = async () => {
        // 这个考虑放repo/home 里面
        const r = await requestTarget<{ data: RepositoryRelease }>(
            'repo/release/detail',
            {
                owner: owner,
                id: object_id,
                user_id: stackInfo.owner,
                tag_name: decodeURIComponent(tag_name)
            },
            owner,
            object_id
        );
        if (r.err) {
            console.log(' fetch repo releae info failed');
            return;
        }
        console.log('r---', r.data!.data);
        const data = r.data!.data;
        setTagName(data.tag_name);
        setTitle(data.title);
        setContent(data.content);
        setBranch(data.tag_target);
        setId(data.id);
    };

    const menuList = (
        <Menu>
            {branches.map((branch: string) => (
                <Menu.Item
                    key={branch}
                    onClick={() => {
                        setBranch(branch);
                    }}
                >
                    {' '}
                    {branch}{' '}
                </Menu.Item>
            ))}
        </Menu>
    );

    const verifyParam = () => {
        console.log('verifyParam---', tagName, title, content);
        if (tagName === '') {
            message.warn(t('warn.release.new.tag'));
            return false;
        }
        if (!checkName(tagName.trim())) {
            message.warn(t('warn.release.new.tag.format'));
            return;
        }
        if (title === '') {
            message.warn(t('warn.release.new.title'));
            return false;
        }
        if (content === '') {
            message.warn(t('warn.release.new.content'));
            return false;
        }
        return true;
    };

    const addSubmit = async () => {
        if (!verifyParam()) {
            return;
        }
        const hide = message.loading('提交中...', 0);
        const r = await requestTarget(
            'repo/release/add',
            {
                owner: owner,
                id: object_id,
                user_id: stackInfo.owner,
                tag_name: tagName,
                tag_target: branch,
                title: title,
                content: content
            },
            owner,
            object_id
        );
        if (r.err) {
            console.log('fetch repo release failed');
            hide();
            message.error(r.msg);
            return;
        }
        hide();
        message.success(t('success.release.new'));
        setTimeout(() => {
            history.push(`/${owner}/${object_id}/releases`);
        }, 1500);
    };

    const editSubmit = async () => {
        console.log('测试', tagName, title, content);
        if (!verifyParam()) {
            return;
        }
        const r = await requestTarget(
            'repo/release/edit',
            {
                owner: owner,
                id: object_id,
                user_id: stackInfo.owner,
                title: title,
                content: content,
                tag_name: tagName
            },
            owner,
            object_id
        );
        if (r.err) {
            console.log('fetch repo release failed');
            message.error(r.msg);
            return;
        }
        message.success(t('success.release.edit'));
        setTimeout(() => {
            history.push(`/${owner}/${object_id}/releases`);
        }, 1000);
    };

    const deleteCallBack = async () => {
        const r = await requestTarget(
            'repo/release/delete',
            {
                release_id: id
            },
            owner,
            object_id
        );
        if (r.err) {
            console.log(' fetch repo release failed');
            message.error(r.msg);
            return;
        }
        message.success(t('success.release.delete'));
        history.push(`/${owner}/${object_id}/releases`);
    };

    const deleteSubmit = () => {
        childCmpRef.current?.setLoading(false);
        childCmpRef.current?.setShow(true);
        setParam({
            title: t('repository.file.release.new.prompt'),
            content: t('repository.file.release.new.prompt.delete'),
            cb: deleteCallBack
        });
    };

    return (
        <div>
            <div className={styles.repoReleaseNewHeader}>
                <h2>{pageTitle}</h2>
                <div className={styles.repoReleaseNewSelect}>
                    <input
                        type="text"
                        className={styles.repoReleaseInput}
                        defaultValue={tagName}
                        disabled={isEdit}
                        onChange={(e) => {
                            setTagName(e.target.value.trim());
                        }}
                        placeholder={t('repository.file.release.new.name')}
                    />
                    <div className={styles.repoReleaseChar}>@</div>
                    <div className={styles.releaseSwitch}>
                        <Dropdown
                            overlay={menuList}
                            placement="bottomCenter"
                            arrow
                            trigger={['click']}
                            disabled={isEdit}
                        >
                            <div className={styles.branchSwitchDropdown}>
                                <img src={ForkIcon} />
                                <span style={{ marginRight: 10 }}>
                                    {t('repository.file.release.new.branch')}:
                                </span>
                                <strong>{branch}</strong>
                                <DownOutlined style={{ marginLeft: 5 }} />
                            </div>
                        </Dropdown>
                    </div>
                </div>
            </div>
            <div className={styles.repoReleaseNewMain}>
                <h3>{t('repository.file.release.new.subtitle')}</h3>
                <input
                    type="text"
                    className={(styles.repoReleaseInput, styles.repoReleaseTitleInput)}
                    placeholder={t('repository.file.release.new.subtitle.input')}
                    defaultValue={title}
                    onChange={(e) => {
                        setTitle(e.target.value);
                    }}
                />
                <h3>{t('repository.file.release.new.content')}</h3>
                {/* <div className="repo-releas-new-editor">
                    <Editor onChange={setContent} value={ content }/>
                </div>                 */}
                <textarea
                    rows={10}
                    placeholder={t('repository.file.release.new.content.input')}
                    defaultValue={content}
                    onChange={(e) => {
                        setContent(e.target.value);
                    }}
                ></textarea>
            </div>
            <div className={styles.repoReleaseNewFooter}>
                {!isEdit && (
                    <button
                        className={styles.repoReleaseBtn}
                        onClick={() => {
                            addSubmit();
                        }}
                    >
                        {t('repository.file.release.new.btn')}
                    </button>
                )}
                {isEdit && (
                    <button
                        className={styles.repoReleaseBtn}
                        onClick={() => {
                            editSubmit();
                        }}
                    >
                        {t('repository.file.tag.update.btn')}
                    </button>
                )}
                {isEdit && (
                    <button
                        className={styles.releaseDeleteBtn}
                        onClick={() => {
                            deleteSubmit();
                        }}
                    >
                        {t('repository.file.tag.delete.btn')}
                    </button>
                )}
                <button
                    className={styles.repoReleaseCancelBtn}
                    onClick={() => {
                        history.push(`/${owner}/${object_id}/releases`);
                    }}
                >
                    {t('repository.file.tag.cancel')}
                </button>
            </div>
            <PromptDialog param={deleteParam} cRef={childCmpRef}></PromptDialog>
        </div>
    );
};

export default withRouter(RepoReleaseNew);
