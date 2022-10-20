import React, { useRef, useState } from 'react';
import { repositoryAtom } from '@src/stores/repository';
import { message } from 'antd';
import { requestLocal } from '@src/utils/request';
import { useHistory, useParams } from 'react-router-dom';
import PromptDialog from '@src/components/PromptDialog';
import { userInfoAtom } from '@src/stores/user';
import styles from './Default.css';
import { useRecoilState } from 'recoil';
import { RepositoryType } from '@src/types/rpc_def';
import { useTranslation } from 'react-i18next';

const RepoSettingDefault: React.FC = () => {
    const history = useHistory();
    const childCmpRef = useRef<PromptDialogRefType>(null);
    const [deleteParam, setParam] = useState<DeleteParam>({ title: '', content: '' });
    const { object_id, owner } = useParams<RepoUrlParams>();
    const [userInfo] = useRecoilState(userInfoAtom);
    const [repository] = useRecoilState(repositoryAtom);
    const { t } = useTranslation();
    const text =
        repository.is_private == RepositoryType.Public ? t('repository.settings.repository.private') : t('repository.settings.repository.public');

    const deleteCallBack = async () => {
        // if (repository.is_private == RepositoryType.Public ) {  // 公开仓库删除
        //     const r = await requestService('repo/delete', {
        //         owner: owner,
        //         id: object_id
        //     } as RequestRepositoryHome)
        //     if (r.err) {
        //         message.error(`fetch dec app data err ${r.msg}`)
        //         return
        //     }
        // }

        const r = await requestLocal('repo/delete', {
            author_name: owner,
            name: object_id
        });
        if (r.err) {
            message.error(r.msg);
            return;
        }
        successPrompt(t('success.repository.setting.delete'));
        setTimeout(() => {
            history.push('/');
        }, 1000);
    };

    const settingCallBack = async () => {
        // 公开仓库设置为私有仓库
        const r = await requestLocal('repo/state/switch', {
            author_name: owner,
            name: object_id
            // owner: owner,
            // id: object_id
        });
        if (r.err) {
            message.error(`fetch dec app data err ${r.msg}`);
            return;
        }

        // const r = await requestTarget('repo/state/switch', {
        //     owner: owner,
        //     id: object_id,
        //     action: repository.is_private === RepositoryType.Public ? 'setprivate' : 'setpublic'
        // }, owner, object_id)
        // if (r.err) {
        //     message.error(`fetch dec app data err ${r.msg}`)
        //     return
        // }
        successPrompt(t('success.repository.setting.status'));
        setTimeout(() => {
            window.location.reload();
        }, 1000);
    };

    const onDelete = () => {
        repositoryOperation(
            t('repository.settings.repository.prompt'),
            `${t('repository.settings.repository.delete.text')} "${repository.name}"?`,
            deleteCallBack
        );
    };

    const onSwitch = () => {
        repositoryOperation(
            t('repository.settings.repository.prompt'),
            `${t('repository.settings.repository.switch.text')}"${repository.name}"${text}?`,
            settingCallBack
        );
    };

    const successPrompt = (text: string) => {
        childCmpRef.current?.setShow(false);
        childCmpRef.current?.setLoading(false);
        message.success(text);
    };

    const repositoryOperation = (title: string, content: string, callback: Function) => {
        childCmpRef.current?.setLoading(false);
        childCmpRef.current?.setShow(true);
        setParam({
            title: title,
            content: content,
            cb: callback
        });
    };

    return (
        <div className={styles.container}>
            <div className={styles.title}>
                <h1>{t('repository.settings.repository.title')}</h1>
            </div>
            <div className={styles.manageBtn}>
                <button className={styles.settingBtn} onClick={onSwitch}>
                    {text}
                </button>
                <button className={styles.repoSettingBtnDelete} onClick={onDelete}>
                    {t('repository.settings.repository.delete')}
                </button>
            </div>
            <PromptDialog param={deleteParam} cRef={childCmpRef}></PromptDialog>
        </div>
    );
};

export default RepoSettingDefault;
