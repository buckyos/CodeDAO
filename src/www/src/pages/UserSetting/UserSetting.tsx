import { Input, Button, message } from 'antd';
import _ from 'lodash';
import React from 'react';
import { useHistory } from 'react-router-dom';
import { useRecoilState } from 'recoil';
import { stackInfo } from '@src/utils/stack';
import { validateEmail } from '../../utils/validate/validate';
import { RequestUserSetting } from '../../types';
import { userInfoAtom } from '../../stores/user';
import { requestLocal } from '../../utils';
import { updateUserInfoCache } from '../../utils/cache';
import { useTranslation } from 'react-i18next';
import styles from './UserSetting.module.less';

const UserSetting: React.FC = () => {
    const history = useHistory();
    const [userInfo, setUserInfo] = useRecoilState(userInfoAtom);
    const [name, setName] = React.useState(userInfo.name);
    const [email, setEmail] = React.useState(userInfo.email);
    const { t } = useTranslation();

    const onSubmit = async () => {
        if (name == '') {
            message.error(t('error.user.setting.name.empty'));
            return;
        }

        if (email == '') {
            message.error(t('error.user.setting.email.empty'));
            return;
        }

        if (!_.isEmpty(email) && !validateEmail(email)) {
            message.error(
                `${t('error.user.setting.email')} [${email}] ${t(
                    'error.user.setting.email.format'
                )}`
            );
            return;
        }

        const req: RequestUserSetting = {
            owner: stackInfo.owner.to_base_58(),
            name: name,
            email: email,
            userId: userInfo.userId
        };
        const r = await requestLocal('user/setting', req);
        if (r.err) {
            console.log('设置用户信息失败', r);
            message.error(`${t('error.user.setting.fail')}: ${r.msg}`);
            return;
        }
        message.success(t('success.user.setting'));
        setUserInfo({
            ...userInfo,
            name,
            email
        });
        updateUserInfoCache(email);
        setTimeout(() => {
            history.push('/');
        }, 1200);
    };

    return (
        <div className={styles.userSettingMain}>
            <h4 className={styles.userInfoSetting}>{t('user.setting.title')}</h4>
            <div className={styles.initRepoContent}>
                {/* <div className={styles.repoPullCreat}"owner-reponame">
                        <label className={styles.repoPullCreat}'require'>用户名字</label>
                        <Input style={{width: 400}} value={name} onChange={e => { console.log('测试'); setName(e.target.value) }}/>
                    </div> */}

                <div className={styles.ownerReponame}>
                    <label className={styles.require}>{t('user.setting.email')}</label>
                    <Input
                        style={{ width: 400 }}
                        value={email}
                        onChange={(e) => setEmail(e.target.value)}
                    />
                </div>

                <div className={styles.createRepoButtons}>
                    <Button className={styles.createBtn} type="primary" onClick={onSubmit}>
                        {t('user.setting.submit')}
                    </Button>
                </div>
            </div>
        </div>
    );
};

export default UserSetting;
