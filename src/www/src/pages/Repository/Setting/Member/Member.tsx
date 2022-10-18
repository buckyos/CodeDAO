import React, { useCallback } from 'react';
import { useParams } from 'react-router-dom';
import { requestLocal, requestTarget } from '@src/utils/request';
import { Dropdown, Menu, message, Tag, Spin } from 'antd';
import { DownOutlined } from '@ant-design/icons';
import UserLogo from '../../../../components/UserLogo/UserLogo';
import { InviteModal } from '../../../../components/Invite/Invite';
import { repositoryAtom } from '../../../../stores/repository';
import { MenuInfo } from 'rc-menu/lib/interface';
import styles from './Member.css';
import { useRequestShadow } from '../../../../utils';
import { useRecoilState } from 'recoil';
import { inviteModalStoreAtom } from '../../../../stores/modal';
import { useTranslation } from 'react-i18next';

const PrivilegeBox: React.FC<PrivilegeBox> = ({ data, onChange }) => {
    const { t } = useTranslation();

    const switchPrivilege = (item: MenuInfo) => {
        onChange(data, item.key);
    };

    const menu = () => (
        <Menu onClick={switchPrivilege}>
            <Menu.Item key="admin">{t('repository.settings.member.permission.admin')}</Menu.Item>
            <Menu.Item key="write">{t('repository.settings.member.permission.write')}</Menu.Item>
            <Menu.Item key="read">{t('repository.settings.member.permission.read')}</Menu.Item>
        </Menu>
    );

    const privilegeType = (target: string) => {
        let str = '';
        switch (target) {
            case 'admin':
                str = t('repository.settings.member.permission.admin');
                break;
            case 'write':
                str = t('repository.settings.member.permission.write');
                break;
            case 'read':
                str = t('repository.settings.member.permission.read');
                break;
        }
        return str;
    };

    return (
        <Dropdown overlay={menu()} placement="bottomCenter" arrow trigger={['click']}>
            <div>
                <Tag>
                    <span style={{ marginRight: 5 }}>{privilegeType(data.role)}</span>
                </Tag>
                <DownOutlined />
            </div>
        </Dropdown>
    );
};

const RepoSettingMember: React.FC = () => {
    const { object_id, owner } = useParams<RepoUrlParams>();
    const [repository] = useRecoilState(repositoryAtom);
    const [inviteModalStore, setInviteModalStore] = useRecoilState(inviteModalStoreAtom);
    const { t } = useTranslation();

    // TODO
    const {
        data,
        loading,
        request: loadMember
    } = useRequestShadow<{ data: ResponseRepositoryMember[] }>(
        () =>
            requestLocal('repo/member', {
                // repository_id: repository.id,
                name: object_id,
                author_name: owner
            }),
        [],
        'get member list failed'
    );
    // console.log('memberList memberList',data)

    React.useEffect(() => {
        loadMember();
    }, []);

    // 添加新成员
    const addRepositoryMember = async (value: string) => {
        // const req: RequestRepositoryMemberAdd = {
        //     repository_id: repository.id,
        //     user_id: value,
        // }
        const r2 = await requestLocal('repo/member/add', {
            name: object_id,
            author_name: owner,
            user_id: value
        });
        if (r2.err) {
            message.error(r2.msg);
            return;
        }

        message.success(t('success.member.new'));
        loadMember();

        // reload
        setTimeout(() => {
            setInviteModalStore({
                ...inviteModalStore,
                show: false
            });
        }, 500);
    };

    // 移除
    const onDeleteMember = useCallback((item: ResponseRepositoryMember) => {
        return async () => {
            // const request: RequestRepositoryMemberDelete = {
            //     id: item.id,
            //     // TODO
            //     repository_id: repository.id,
            //     user_id: item.user_id,
            // }
            const r = await requestLocal('repo/member/delete', {
                name: object_id,
                author_name: owner,
                user_id: item.user_id
            });
            if (r.err) {
                message.error(r.msg);
                return;
            }
            message.success(t('success.member.delete'));
            loadMember();
        };
    }, []);

    // 更改role权限
    const onChangeRole = async (item: ResponseRepositoryMember, value: string) => {
        // const request: RequestRepositoryMemberChangeRole = {
        //     role: value,
        //     id: data.id,
        //     repository_id: repository.id,
        //     user_id: data.user_id,
        // }
        const r = await requestLocal('repo/member/role', {
            name: object_id,
            author_name: owner,
            user_id: item.user_id,
            role: value
        });
        if (r.err) {
            message.error(r.msg);
            return;
        }
        message.success(t('success.member.edit'));
        loadMember();
    };

    return (
        <div>
            <div className={styles.title}>
                <h1>{t('repository.settings.member.title')}</h1>
            </div>
            <div className={styles.friendManage}>
                <div className={styles.friendManageHeader}>
                    {t('repository.settings.member.collaborators.title')}
                </div>
                <div className="friend-manage-list">
                    {loading && <Spin></Spin>}
                    {!loading &&
                        data &&
                        data.data.map((item: ResponseRepositoryMember, key) => (
                            <div className={styles.friendManageItem} key={key}>
                                <UserLogo
                                    className={styles.headerUserLogo}
                                    user_id={item.user_id}
                                />
                                <div className={styles.friendId}>{item.user_name}</div>
                                <div className={styles.friendPrivilege}>
                                    <PrivilegeBox onChange={onChangeRole} data={item} key={key} />
                                </div>
                                <button
                                    onClick={onDeleteMember(item)}
                                    className={styles.friendDelete}
                                >
                                    {t('repository.settings.member.collaborators.delete')}
                                </button>
                            </div>
                        ))}
                </div>
                <div className={styles.friendManageBottom}>
                    <button
                        onClick={() =>
                            setInviteModalStore({
                                ...inviteModalStore,
                                show: true
                            })
                        }
                    >
                        {t('repository.settings.member.collaborators.add')}
                    </button>
                </div>
            </div>
            <InviteModal onConfirm={addRepositoryMember} />
        </div>
    );
};

export default RepoSettingMember;
