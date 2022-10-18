import React from 'react';
import { Dropdown, Menu } from 'antd';
import { DownOutlined, PlusOutlined, SettingOutlined, UserOutlined } from '@ant-design/icons';
import { useHistory } from 'react-router-dom';
import UserLogo from '../UserLogo/UserLogo';
import { useRecoilState } from 'recoil';
import { userInfoAtom } from '../../stores/user';
import { useTranslation } from 'react-i18next';
import styles from './HeaderLogin.css';

const HeaderLogin: React.FC = () => {
    const history = useHistory();
    const [userInfo] = useRecoilState(userInfoAtom);
    const { t } = useTranslation();

    const menu = (
        <Menu>
            <Menu.Item
                onClick={() => {
                    history.push('/create/repo');
                }}
            >
                {t('header.create.repository')}
            </Menu.Item>
            <Menu.Item
                onClick={() => {
                    history.push('/create/organization');
                }}
            >
                {t('header.create.organization')}
            </Menu.Item>
        </Menu>
    );

    const userCenter = (
        <Menu>
            <Menu.Item
                onClick={() => {
                    history.push(`/userInfo/${userInfo.name}/${userInfo.id}`);
                }}
            >
                <UserOutlined style={{ marginRight: 5 }} />
                {t('header.user.info')}
            </Menu.Item>
            <Menu.Item
                onClick={() => {
                    history.push('/user_setting');
                }}
            >
                <SettingOutlined style={{ marginRight: 5 }} />
                {t('header.user.setting')}
            </Menu.Item>
        </Menu>
    );

    return (
        <div className={styles.headerRightWrap}>
            <Dropdown overlay={menu} placement="bottomCenter" arrow trigger={['click']}>
                <div className={styles.headerRightDropdown}>
                    <PlusOutlined style={{ marginRight: 5 }} />
                    <DownOutlined />
                </div>
            </Dropdown>
            <Dropdown overlay={userCenter} placement="bottomCenter" arrow trigger={['click']}>
                <div className={styles.headerRightDropdown}>
                    <UserLogo user_id={userInfo.id} />
                    <span className={styles.headerRightItem} style={{ marginRight: 5 }}>
                        {userInfo.name}
                    </span>
                    <DownOutlined />
                </div>
            </Dropdown>
        </div>
    );
};

export default HeaderLogin;
