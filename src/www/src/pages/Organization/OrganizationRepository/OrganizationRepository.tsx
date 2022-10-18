import React, { useCallback, useEffect, useState } from 'react';
import { useHistory } from 'react-router-dom';
import { useRecoilState } from 'recoil';
import { requestLocal } from '@src/utils/request';
import { userInfoAtom } from '../../../stores/user';
import { toYMDHMS } from '../../../utils';
import PrivateImg from '@src/assets/images/private.png';
import OpenImg from '@src/assets/images/open.png';
import styles from './OrganizationRepository.css';
import { useTranslation } from 'react-i18next';

const OrganizationRepository: React.FC<OrganizationRepositoryProps> = ({ organization }) => {
    const [repositorys, setRepositorys] = useState<ResponseRepository[]>([]);
    // const [members, setMembers ]  =  useState<ResponseOrganizationMember[]>([])
    // const [userInfo] = useRecoilState(userInfoAtom)
    const history = useHistory();
    const { t } = useTranslation();

    useEffect(() => {
        (async () => {
            // const res = await reloadMember(organization)
            const req: RequestOrganizationRepository = { name: organization.name };
            const r = await requestLocal<{ data: ResponseRepository[] }>(
                'organization/repository',
                req
            );
            if (r.err || r.data === undefined) {
                console.error('get org failed', r);
                return;
            }
            console.log('ResponseOrganizationList', r.data);
            // const filterData = r.data.data.filter((ff: ResponseRepository) => ff.is_private === 0)  // 非组织成员访问的仓库列表
            // const data = (res || []).filter(ff => ff.user_id === userInfo.owner).length > 0 ? r.data.data : filterData
            setRepositorys(r.data.data);
        })();
    }, [organization]);

    // const reloadMember = async (org: ResponseOrganizationHome) => {
    //     const r2 = await requestService<{data: ResponseOrganizationMember[]}>('organization/member', { organization_id: org.id} as RequestOrganizationMember)
    //     if (r2.err || r2.data === undefined) {
    //         console.error('get members failed', r2)
    //         return
    //     }
    //     return r2.data.data
    // }

    const locationRepository = useCallback((item: ResponseRepository) => {
        history.push(`/${item.author_name}/${item.name}`);
    }, []);

    return (
        <div className={styles.main}>
            <div className={styles.repoHeader}>{t('organization.detail.repository.title')}</div>
            {repositorys.map((repository: ResponseRepository) => (
                <div className={styles.userInfoRepoItem} key={repository.id}>
                    <div className={styles.userInfoRepoItemLeft}>
                        {
                            <img
                                src={repository.is_private == 1 ? PrivateImg : OpenImg}
                                className={styles.repositoryLogo}
                                alt=""
                            />
                        }
                    </div>
                    <div className={styles.userInfoRepoItemRight}>
                        <div
                            className={styles.userInfoRepoItemName}
                            onClick={() => {
                                locationRepository(repository);
                            }}
                        >
                            {repository.author_name}/{repository.name}
                        </div>
                        <div className={styles.userInfoRepoItemTime}>
                            {t('organization.detail.repository.creation')}：
                            {toYMDHMS(repository.date!)}
                        </div>
                    </div>
                </div>
            ))}
        </div>
    );
};

export default OrganizationRepository;
