import React from 'react';
import { useHistory } from 'react-router-dom';
import { message, Input } from 'antd';
import { requestLocal } from '@src/utils/index';
import { RequestOrganizationCreate } from '@src/types';
import { validateEmail, validateName } from '@src/utils/validate/validate';
import _ from 'lodash';
import { useTranslation } from 'react-i18next';
import styles from './OrganizationNew.module.less';

const OrganizationNew: React.FC = () => {
    const history = useHistory();
    const [name, setName] = React.useState('');
    const [email, setEmail] = React.useState('');
    const { t } = useTranslation();

    const onSubmit = async () => {
        if (name == '') {
            message.error(t('error.organization.new.name.empty'));
            return;
        }

        if (!_.isEmpty(name) && !validateName(name)) {
            message.error(
                `${t('error.organization.new.name')} [${name}] ${t(
                    'error.organization.new.format'
                )}`
            );
            return;
        }

        if (email == '') {
            message.error(t('error.organization.new.email.empty'));
            return;
        }

        if (!_.isEmpty(email) && !validateEmail(email)) {
            message.error(
                `${t('error.organization.new.email')} [${email}] ${t(
                    'error.organization.new.email.format'
                )}`
            );
            return;
        }

        const req: RequestOrganizationCreate = {
            name: name,
            email: email
        };
        const r = await requestLocal('organization/new', req);
        if (r.err) {
            console.log('org new failed ', r);
            message.error(`${t('error.organization.new.fail')}: ${r.msg}`);
            return;
        }

        message.success(t('success.organization.new'));
        setTimeout(() => {
            history.push('/explore/organizations');
        }, 1500);
    };

    return (
        <div className={styles.detailMain}>
            <div className={styles.newOrganizationContent}>
                <h4 className={styles.emptyOrganizationHeader}>{t('create.organization.title')}</h4>
                <div className={styles.ownerOrganizationName}>
                    <label className={styles.require}>{t('create.organization.name')}：</label>
                    <Input
                        style={{ width: 400 }}
                        value={name}
                        onChange={(e) => setName(e.target.value)}
                    />
                </div>
                <div className={styles.ownerOrganizationName}>
                    <label className={styles.require}>{t('create.organization.email')}：</label>
                    <Input
                        style={{ width: 400 }}
                        value={email}
                        onChange={(e) => setEmail(e.target.value)}
                    />
                </div>

                <div className={styles.createOrganizationButtons}>
                    <button className={styles.createBtn} onClick={onSubmit}>
                        {t('create.organization.new')}
                    </button>
                    <button
                        className={styles.cancelBtn}
                        onClick={() => {
                            history.push('/');
                        }}
                    >
                        {t('create.organization.cancel')}
                    </button>
                </div>
            </div>
        </div>
    );
};

export default OrganizationNew;
