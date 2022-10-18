import React from 'react';
import { useHistory } from 'react-router-dom';
import { Input, Button, message } from 'antd';
import { stackInfo } from '@src/utils/stack';
import { requestLocal } from '../../utils';
import { RequestUserInit, ResponseCheckUser } from '@src/types/index';
import _ from 'lodash';
import { validateEmail, validateName } from '../../utils/validate/validate';
import { useTranslation } from 'react-i18next';
import styles from './InitUser.css';

const InitUser: React.FC = () => {
    const history = useHistory();
    const [name, setName] = React.useState('');
    const [email, setEmail] = React.useState('');
    const { t } = useTranslation();

    React.useLayoutEffect(() => {
        (async function () {
            const r = await requestLocal<ResponseCheckUser>('user/checkInit', {
                owner: stackInfo.owner.to_base_58()
            });
            if (r.err) {
                console.error('request user/checkInit failed', r.msg);
                return;
            }
            if (r.data!.userInit) {
                history.push('/');
            }
        })();
    }, []);

    const onSubmit = async () => {
        if (name == '') {
            message.error(t('error.user.new.name.empty'));
            return;
        }

        if (email == '') {
            message.error(t('error.user.new.email.empty'));
            return;
        }

        if (!_.isEmpty(name) && !validateName(name)) {
            message.error(
                `${t('error.user.new.name')} [${name}] ${t('error.user.new.name.format')}`
            );
            return;
        }

        if (!_.isEmpty(email) && !validateEmail(email)) {
            message.error(
                `${t('error.user.new.email')} [${email}] ${t('error.user.new.email.format')}`
            );
            return;
        }

        const req: RequestUserInit = {
            owner: stackInfo.owner.to_base_58(),
            name: name,
            email: email
        };
        const r = await requestLocal('user/init', req);
        if (r.err) {
            console.log('创建用户失败', r);
            message.error(`${t('error.user.new.fail')}: ${r.msg}`);
            return;
        }
        message.success(t('success.user.new'));
        setTimeout(() => {
            history.push('/');
        }, 1200);
    };

    return (
        <div className={styles.initUserMain}>
            <h4 className={styles.userInfoSetting}>{t('user.init.information')}</h4>
            <div className={styles.initRepoContent}>
                <div className={styles.ownerReponame}>
                    <label className={styles.require}>{t('user.init.name')}</label>
                    <Input
                        style={{ width: 400 }}
                        value={name}
                        onChange={(e) => setName(e.target.value)}
                    />
                </div>

                <div className={styles.ownerReponame}>
                    <label className={styles.require}>{t('user.init.email')}</label>
                    <Input
                        style={{ width: 400 }}
                        value={email}
                        onChange={(e) => setEmail(e.target.value)}
                    />
                </div>

                <div className={styles.createRepoButtons}>
                    <Button className={styles.createBtn} type="primary" onClick={onSubmit}>
                        {t('user.init.submit')}
                    </Button>
                </div>
            </div>
        </div>
    );
};

export default InitUser;
