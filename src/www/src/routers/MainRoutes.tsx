import React, { useEffect } from 'react';
import { Switch, Route, useHistory } from 'react-router-dom';
import { Spin, message } from 'antd';
import { requestLocal } from '@src/utils/index';
import { stack, stackInfo } from '@src/utils/stack';
import { userInfoAtom, UserInfoStore } from '@src/stores/user';
import { ResponseCheckUser, RequestUserInit } from '@src/types';
import { useRecoilState } from 'recoil';
import { mainRouters } from './main';
import { useTranslation } from 'react-i18next';

const AppRoutes: React.FC = () => {
    const history = useHistory();
    const { t } = useTranslation();
    // check user
    const [userInfo, setUserInfo] = useRecoilState(userInfoAtom);
    const init = async () => {
        const r = await requestLocal<ResponseCheckUser>('user/checkInit', {
            owner: stackInfo.owner.to_base_58()
        });
        if (r.err || r.data === undefined) {
            console.error('requestLocal user/checkInit failed', r.msg);
            return;
        }
        if (!r.data.userInit) {
            const req: RequestUserInit = {
                owner: stackInfo.owner.to_base_58(),
                name: '',
                email: ''
            };
            const rs = await requestLocal('user/init', req);
            if (rs.err) {
                console.log('创建用户失败', r);
                message.error(`${t('error.user.new.fail')}: ${r.msg}`);
                return;
            }
            message.success(t('success.user.new'));
            await init();
            return;
        }
        if (r.data.user) {
            setUserInfo({
                ...r.data.user,
                id: stackInfo.owner.toString()
            });
            UserInfoStore.setData(r.data.user, stackInfo.owner.toString());
        }
    };
    useEffect(() => {
        init();
    }, []);

    return (
        <Switch>
            {mainRouters.map((route) => {
                return (
                    <Route exact={route.exact} path={route.path} key={route.path}>
                        {<route.component />}
                    </Route>
                );
            })}
        </Switch>
    );
};

export default AppRoutes;
