import React from 'react';
import { useHistory, useParams } from 'react-router-dom';
import { branchesAtom } from '@src/stores/repository';
import { useRecoilState } from 'recoil';
import { useTranslation } from 'react-i18next';
import { ForkOutlined } from '@ant-design/icons';
import styles from './RepoHomeCodeBranch.css';

const RepoHomeCodeBranch: React.FC = () => {
    const { owner, object_id } = useParams<RepoUrlParams>();
    const history = useHistory();
    const [branches] = useRecoilState(branchesAtom);
    const { t } = useTranslation();

    const locationBranchs = () => {
        history.push(`/${owner}/${object_id}/branches`);
    };

    return (
        <div className={styles.repoDetailCommiItem} onClick={locationBranchs}>
            <ForkOutlined className={styles.repoBranch} alt="" />
            <span>{branches.length}</span>
            <dd>{t('repository.file.branches')}</dd>
        </div>
    );
};

export default RepoHomeCodeBranch;
