import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';
import en from './locales/en';
import zh from './locales/zh';

const resources = {
    en: {
        translation: en
    },
    zh: {
        translation: zh
    }
};

// console.log('语言资源>>>>', JSON.stringify(resources));

i18n.use(LanguageDetector)
    .use(initReactI18next)
    .init({
        fallbackLng: 'zh',
        lng: 'en',
        debug: true,
        resources,
        interpolation: {
            escapeValue: false
        }
    });

export default i18n;
