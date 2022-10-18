import React from 'react';
import UserLogo1Icon from '@src/assets/images/user_logo_1.png';
import UserLogo2Icon from '@src/assets/images/user_logo_2.png';
import UserLogo3Icon from '@src/assets/images/user_logo_3.png';
import styles from './OrganizationLogo.css';

const OrganizationLogo: React.FC<UserLogoProps> = ({ user_id, className }) => {
    const getLogoUrl = (id: string) => {
        const number = id.charCodeAt(id.length - 1) % 3;
        const logoUrlList = [UserLogo1Icon, UserLogo2Icon, UserLogo3Icon];
        return logoUrlList[number];
    };

    return (
        <img className={className ? className : styles.headerUserLogo} src={getLogoUrl(user_id)} />
    );
};

export default OrganizationLogo;
