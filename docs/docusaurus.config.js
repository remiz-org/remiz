// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
    title: 'Remiz',
    tagline: 'Build and deploy package, the easy way',
    url: 'https://remiz-org.github.io',
    baseUrl: '/remiz/',
    onBrokenLinks: 'throw',
    onBrokenMarkdownLinks: 'warn',
    favicon: 'img/favicon.ico',
    organizationName: 'remiz', // Usually your GitHub org/user name.
    projectName: 'remiz', // Usually your repo name.

    presets: [
        [
            'classic',
            /** @type {import('@docusaurus/preset-classic').Options} */
            ({
                docs: {
                    sidebarPath: require.resolve('./sidebars.js'),
                    // Please change this to your repo.
                    editUrl: 'https://remiz-org.github.io/remiz/main/docs/',
                },
                theme: {
                    customCss: require.resolve('./src/css/custom.css'),
                },
            }),
        ],
    ],

    themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
        ({
        navbar: {
            title: 'Remiz',
            logo: {
                alt: 'Remiz Logo',
                src: 'img/logo.svg',
            },
            items: [{
                    type: 'doc',
                    docId: 'intro',
                    position: 'left',
                    label: 'Tutorial',
                },
                {
                    type: 'localeDropdown',
                    position: 'right',
                },
                {
                    href: 'https://github.com/remiz-org/remiz',
                    label: 'GitHub',
                    position: 'right',
                },
            ],
        },
        footer: {
            style: 'dark',
            links: [{
                    title: 'Docs',
                    items: [{
                        label: 'Tutorial',
                        to: '/docs/intro',
                    }, ],
                },
                {
                    title: 'Community',
                    items: [{
                            label: 'Stack Overflow',
                            href: 'https://stackoverflow.com/',
                        },
                        {
                            label: 'Discord',
                            href: 'https://discordapp.com/',
                        },
                        {
                            label: 'Twitter',
                            href: 'https://twitter.com/',
                        },
                    ],
                },
                {
                    title: 'More',
                    items: [{
                        label: 'GitHub',
                        href: 'https://github.com/remiz-org/remiz',
                    }, ],
                },
            ],
            copyright: `Copyright © ${new Date().getFullYear()} Remiz, Inc.`,
        },
        prism: {
            theme: lightCodeTheme,
            darkTheme: darkCodeTheme,
        },
    }),
    i18n: {
        defaultLocale: 'en',
        locales: ['en', 'fr'],
    },
};

module.exports = config;