import React from 'react';
import UserLogo1Icon from '@src/assets/images/user_logo_1.png';
import UserLogo2Icon from '@src/assets/images/user_logo_2.png';
import UserLogo3Icon from '@src/assets/images/user_logo_3.png';
import styles from './UserLogo.css';

const UserLogo: React.FC<UserLogoProps> = ({ user_id, className }) => {
    const getLogoUrl = (id: string) => {
        const number = id.charCodeAt(id.length - 1) % 3;
        const logoUrlList = [UserLogo1Icon, UserLogo2Icon, UserLogo3Icon];
        return logoUrlList[number];
    };

    return (
        <img
            className={className ? className : styles.headerUserLogo}
            data-people-id={user_id}
            src={getLogoUrl(user_id)}
        />
    );
};

export default UserLogo;
