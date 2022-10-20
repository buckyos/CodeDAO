import React from 'react';
import { useHistory, useParams } from 'react-router-dom';
import { useRecoilState } from 'recoil';
import { useTranslation } from 'react-i18next';
import { repositoryAtom, repositorySettingAtom } from '@src/stores/repository';
import FileIcon from '@src/assets/images/file.png';
import FileCheckedIcon from '@src/assets/images/file_checked.png';
import IssueIcon from '@src/assets/images/issue.png';
import IssueCheckedIcon from '@src/assets/images/issue_checked.png';
import MergeIcon from '@src/assets/images/merge.png';
import MergeCheckedIcon from '@src/assets/images/merge_checked.png';
import WikigIcon from '@src/assets/images/wiki.png';
import WikiCheckedIcon from '@src/assets/images/wiki_checked.png';
import SettingIcon from '@src/assets/images/setting.png';
import SettingCheckedIcon from '@src/assets/images/setting_checked.png';
import styles from './RepoHomeMenus.module.less';

const RepoHomeMenus: React.FC = () => {
    const { owner, object_id } = useParams<RepoUrlParams>();
    const [activeKey, setActiveKey] = React.useState('/');
    const [is_setting] = useRecoilState(repositorySettingAtom);
    const history = useHistory();
    const { t } = useTranslation();

    // const isAdmin = userInfo.owner === owner
    const defaultTabs: RepositoryTabType[] = [
        {
            name: t('repository.tab.file'),
            key: '/',
            src: FileIcon,
            checkedSrc: FileCheckedIcon
        },
        {
            name: t('repository.tab.issue'),
            key: '/issues',
            src: IssueIcon,
            checkedSrc: IssueCheckedIcon
        },
        {
            name: t('repository.tab.pull.request'),
            key: '/pulls',
            src: MergeIcon,
            checkedSrc: MergeCheckedIcon
        },
        {
            name: t('repository.tab.wiki'),
            key: '/wiki',
            src: WikigIcon,
            checkedSrc: WikiCheckedIcon
        },
        {
            name: t('repository.tab.setting'),
            hide: !is_setting,
            key: '/setting',
            src: SettingIcon,
            checkedSrc: SettingCheckedIcon
        }
    ];
    const [tabs, setTabs] = React.useState<RepositoryTabType[]>(defaultTabs);
    React.useEffect(() => {
        setTabs([...defaultTabs]);
    }, [is_setting]);

    // React.useEffect(()=> {
    //     (async () => {
    //         const req: RequestRepositoryMemberGet = {
    //             repository_id: object_id,
    //             user_id: stackInfo.owner.to_base_58(),
    //         }
    //         const r = await requestTarget<{data?: ResponseRepositoryMember}>('repository/member/get', req, owner, object_id)
    //         if (r.err || r.data === undefined) {
    //             message.error('get user info failed')
    //             return
    //         }
    //         if (r.data.data && r.data.data.role === 'admin') {
    //             defaultTabs[defaultTabs.length - 1].hide = false
    //         }
    //
    //         // 这里需要deep clone，不然不触发render
    //         setTabs([...defaultTabs])
    //         console.log('defaultTabs', defaultTabs)
    //     })()
    // }, [])

    // 设置当前选中的tab高亮
    React.useEffect(() => {
        const current = `/${owner}/${object_id}`;

        // 整体是hashRouter 这里也要用location.hash 去做判断
        const tail_path = location.hash.replace(current, '');
        for (const tab of tabs) {
            if (tab.key == '/') {
                continue;
            }
            // console.log(tail_path.match(tab.key)  )
            if (tail_path.match(tab.key)) {
                setActiveKey(tab.key);
                break;
            }
        }
    }, []);

    const onChange = (key: string) => {
        setActiveKey(key);

        if (key == '/') {
            key = key.replace('/', '');
        }

        history.push(`/${owner}/${object_id}${key}`);
    };
    return (
        <div className={styles.repoDetailHeader2}>
            <div className={styles.repoTab}>
                {tabs
                    .filter((tab) => tab.hide != true)
                    .map((tab) => {
                        const name =
                            activeKey === tab.key ? styles.repoTabItemActive : styles.repoTabItem;
                        return (
                            <div
                                key={tab.name}
                                className={name}
                                onClick={() => {
                                    onChange(tab.key);
                                }}
                            >
                                {activeKey === tab.key ? (
                                    <img src={tab.checkedSrc} alt="" />
                                ) : (
                                    <img src={tab.src} alt="" />
                                )}
                                <span>{tab.name}</span>
                            </div>
                        );
                    })}
            </div>
        </div>
    );
};

export default RepoHomeMenus;
