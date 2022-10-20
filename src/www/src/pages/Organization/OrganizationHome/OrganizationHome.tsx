import React, { useEffect, useState } from 'react';
import { requestLocal } from '@src/utils';
import { useParams, Switch, Route, useHistory } from 'react-router-dom';
import { message, Spin } from 'antd';
import UserLogo from '@src/components/UserLogo/UserLogo';
import OrganizationLogo from '@src/components/OrganizationLogo/OrganizationLogo';
import { InviteModal } from '@src/components/Invite/Invite';
import OrganizationRepository from '../OrganizationRepository/OrganizationRepository';
import styles from './OrganizationHome.module.less';
import { useRecoilState } from 'recoil';
import { userInfoAtom } from '@src/stores/user';
import { inviteModalStoreAtom } from '@src/stores/modal';
import { useTranslation } from 'react-i18next';

const OrganizationHome: React.FC = () => {
    const { name } = useParams<OrganizationParams>();
    const [organization, setOrganization] = useState<ResponseOrganizationHome>();
    const [members, setMembers] = useState<ResponseOrganizationMember[]>([]);
    const [repositoryNumber, setRepositoryNumber] = useState<number>(0);
    const [userInfo] = useRecoilState(userInfoAtom);
    const history = useHistory();
    const [isShow, setIsShow] = useState(false);
    const [inviteModalStore, setInviteModalStore] = useRecoilState(inviteModalStoreAtom);
    const { t } = useTranslation();

    useEffect(() => {
        (async () => {
            const r = await requestLocal<ResponseOrganizationHome>('organization/home', {
                name: name
            });
            if (r.err || r.data === undefined) {
                console.error('get org failed', r);
                return;
            }
            const org = r.data;

            setIsShow(org.is_show_add);
            setOrganization(org);
            const membersRes = await reloadMember(org);
            setMembers(membersRes! || []);
            // setRepositoryNumber(org)

            // reloadMember(org).then()

            // {
            //     const r = await requestLocal<{ data: ResponseRepository[] }>(
            //         'organization/repository',
            //         { name: name},
            //     )
            //     if (r.err || r.data === undefined) {
            //         console.error('get org failed', r)
            //         return
            //     }
            //     console.log('members---', r.data.data)
            // //     const filterData = r.data.data.filter((ff: ResponseRepository) => ff.is_private === 0)  // 非组织成员访问的仓库列表
            // //     const data = membersRes!.filter(ff => ff.user_id === userInfo.owner).length > 0 ? r.data.data : filterData
            //     setRepositoryNumber(r.data.data.length)
            // //     setIsShow(org.ownerId === userInfo.owner)
            // }
        })();
        return () => {
            setInviteModalStore({ ...inviteModalStore, show: false });
        };
    }, [name]);

    const reloadMember = async (org: ResponseOrganizationHome) => {
        const r2 = await requestLocal<{ data: ResponseOrganizationMember[] }>(
            'organization/member',
            { name: name }
        );
        if (r2.err || r2.data === undefined) {
            console.error('get members failed', r2);
            return;
        }
        const data = r2.data.data;
        return data;
    };

    const onInvite = React.useCallback(() => {
        setInviteModalStore({
            ...inviteModalStore,
            title: `Invite a new member join '${organization!.name}'`,
            show: true
        });
    }, [organization]);

    const addOrganizationUser = async (value: string) => {
        if (organization) {
            // console.log(value)
            const r2 = await requestLocal('organization/member/add', {
                name: name,
                user_id: value
            });
            if (r2.err) {
                message.error(r2.msg);
                return;
            }
            message.success(t('success.organization.member.add'));
            // reload
            const members = await reloadMember(organization);
            setMembers(members! || []);

            setTimeout(() => {
                setInviteModalStore({
                    ...inviteModalStore,
                    show: false
                });
            }, 500);
        }
    };

    if (organization == undefined) {
        return <Spin></Spin>;
    }

    return (
        <div className={styles.box}>
            <div className={styles.header}>
                <div className={styles.headerInner}>
                    <OrganizationLogo className={styles.icon} user_id={organization.id} />
                    <div className={styles.name}>{organization.name}</div>
                    <div style={{ flex: 1 }}></div>
                    <div className={styles.headerRight}>
                        <div
                            className={styles.menu}
                            onClick={() => history.push(`/organization/${organization?.name}`)}
                        >
                            {t('organization.detail.member')}
                            <span>{organization.member_count}</span>
                        </div>
                        <div
                            className={styles.menu}
                            onClick={() => history.push(`/organization/${organization?.name}/repo`)}
                        >
                            {t('organization.detail.project')}
                            <span>{organization.repository_count}</span>
                        </div>
                    </div>
                </div>
            </div>

            <Switch>
                <Route exact path="/organization/:name">
                    {isShow && (
                        <div className={styles.main1}>
                            <div onClick={onInvite} className={styles.addButton}>
                                {t('organization.detail.member.invite')}
                            </div>
                        </div>
                    )}
                    <div className={styles.main}>
                        {members.map((item, index) => {
                            return (
                                <div className={styles.row} key={index}>
                                    <div className={styles.user}>
                                        <UserLogo
                                            className={styles.userLogo}
                                            user_id={item.user_id}
                                        />
                                        <div>{item.user_name}</div>
                                    </div>
                                    <div className={styles.column}>
                                        <div className={styles.tr}>
                                            {t('organization.detail.member.visibility')}
                                        </div>
                                        <div>
                                            {t('organization.detail.member.visibility.public')}
                                        </div>
                                    </div>
                                    <div style={{ flex: 1 }}></div>
                                    <div className={styles.column}>
                                        <div className={styles.tr}>
                                            {t('organization.detail.member.role')}
                                        </div>
                                        <div>{item.role}</div>
                                    </div>
                                </div>
                            );
                        })}
                    </div>
                </Route>
                <Route path="/organization/:name/repo">
                    <OrganizationRepository organization={organization} />
                </Route>
            </Switch>

            <InviteModal onConfirm={addOrganizationUser} />
        </div>
    );
};

export default OrganizationHome;
