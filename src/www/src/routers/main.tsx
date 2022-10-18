import React, { FC } from 'react';
import RepoPanel from '@src/components/RepoPanel/RepoPanel';
import RepoActivity from '@src/components/RepoActivity/RepoActivity';
import RepoNew from '@src/pages/RepoNew/RepoNew';
import OrganizationNew from '@src/pages/Organization/OrganizationNew/OrganizationNew';
import Explore from '@src/pages/Explore/Explore/Explore';
import Pulls from '@src/pages/Pulls/Pulls';
import Issues from '@src/pages/Issues/Issues';
import PersonalPage from '@src/pages/PersonalPage/PersonalPage';
import UserInfo from '@src/pages/UserInfo/UserInfo';
import RepoHome from '@src/pages/Repository/RepoHome/RepoHome';
import UserSetting from '@src/pages/UserSetting/UserSetting';
import OrganizationHome from '@src/pages/Organization/OrganizationHome/OrganizationHome';

interface Router {
    path: string;
    exact?: boolean;
    component: FC;
}

const MainPage = () => {
    return (
        <div>
            <div className="content">
                <RepoPanel />
            </div>
            <RepoActivity />
        </div>
    );
};
export const mainRouters: Router[] = [
    {
        path: '/',
        exact: true,
        component: MainPage
    },
    {
        path: '/create/repo',
        component: RepoNew
    },
    {
        path: '/create/organization',
        component: OrganizationNew
    },
    {
        path: '/explore/',
        component: Explore
    },
    {
        path: '/pulls',
        component: Pulls
    },
    {
        path: '/issue',
        component: Issues
    },
    {
        path: '/personal_page/:people_id',
        component: PersonalPage
    },
    {
        path: '/userInfo/:name/:owner',
        component: UserInfo
    },
    {
        path: '/user_setting',
        component: UserSetting
    },
    {
        path: '/organization/:name',
        component: OrganizationHome
    },
    {
        path: '/:owner/:object_id/blob/:branch/:file',
        component: RepoHome
    },
    {
        path: '/:owner/:object_id/commits/:branch',
        component: RepoHome
    },
    {
        path: '/:owner/:object_id/tree/:branch/',
        component: RepoHome
    },
    {
        path: '/:owner/:object_id/src/:branch',
        component: RepoHome
    },
    {
        path: '/:owner/:object_id',
        component: RepoHome
    }
];
