import { Modal } from 'antd';
import React, { useImperativeHandle } from 'react';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';

const PromptDialog: React.FC<{ param: DeleteParam, cRef: React.Ref<PromptDialogRefType> }> = ({ param, cRef }) => {
    const [visible, setVisible] = useState(false);
    const [confirmLoading, setConfirmLoading] = useState(false);
    const { t } = useTranslation();

    const handleOk = async () => {
        setConfirmLoading(true);
        if (param.cb) {
            param.cb(param);
        }
    };

    const setShow = (v: boolean) => {
        setVisible(v);
    };

    const setLoading = (v: boolean) => {
        setConfirmLoading(v);
    };

    const handleCancel = () => {
        setVisible(false);
    };

    useImperativeHandle(cRef, () => ({
        setShow,
        setLoading
    }));

    return (
        <div>
            <Modal
                title={param.title}
                visible={visible}
                onOk={handleOk}
                confirmLoading={confirmLoading}
                onCancel={handleCancel}
                okText={ t('prompt.sure') }
                cancelText={ t('prompt.cancel') }
            >
                <p>{param.content}</p>
            </Modal>
        </div>
    );
};

export default PromptDialog;