# Wutong Contributor Guide

Thank you for your interest and support in the Wutong project! Wutong is a command-line tool designed to enhance developer efficiency, and we warmly welcome your contributions, whether it's reporting issues, suggesting improvements, or directly participating in code development. Here are some guidelines to help you better engage in the construction of the Wutong project.

> [!IMPORTANT]
> We require all contributors to read and sign the [Contributor License Agreement](./docs/CLA.md).

## Wutong Design Philosophy: Focus, Efficiency, and Integration

In the process of software development, developers often need to handle various data conversion and encoding tasks. However, these tasks often require them to step out of their efficient development environment, search for external tools or websites, and engage in cumbersome interactive operations. This is not only time-consuming but also easily disrupts workflow, affecting concentration and inspiration.

The design philosophy of Wutong aims to solve this problem by pursuing the following core values:

1. **Focus**: Wutong allows developers to complete data conversion and encoding tasks directly in the terminal without leaving their development environment, avoiding frequent context switching and maintaining workflow continuity and focus.

2. **Efficiency**: Wutong integrates various common data conversion and encoding functions in the form of a command-line tool, enabling developers to accomplish the most work in the least amount of time and enhance coding efficiency.

3. **Integration**: Wutong seamlessly integrates into the development environment and is compatible with tools and frameworks commonly used by developers, allowing developers to easily use Wutong in their existing workflows without additional adaptation and learning costs.

In summary, Wutong's design philosophy revolves around enhancing developer efficiency and maintaining focus. It strives to create a seamless, efficient, and integrated development environment that allows developers to focus more on the code itself and create more outstanding software products.

## How to Contribute

### 1. Report Issues

If you encounter any issues while using Wutong, please submit a new issue in the project's [Issues](https://github.com/WutongDev/wutong/issues). When submitting, please provide the following information as much as possible:

- **Problem Description**: Describe the problem you encountered in detail.
- **Reproduction Steps**: Provide steps to reproduce the problem.
- **Expected Result**: Describe the result you expect to obtain.
- **Actual Result**: Describe the result you actually obtained.
- **Environment Information**: Include information such as the operating system and Wutong version.

### 2. Suggest Features

If you believe Wutong lacks certain features or if you have suggestions for improving existing features, please also submit a new [Issue](https://github.com/WutongDev/wutong/issues).

Before submitting, please ensure:
- You have searched and **not** found similar issues.
- You have searched the documentation and **not** found relevant content.
- You have tried using the **latest version**, and the issue persists.
- You have **not** modified or replaced any program files.

In your suggestion, please include the following information:
- **Feature Description**: Describe in detail the feature you wish to add or improve.
- **Usage Scenario**: Provide scenarios where this feature would be used in actual development.
- **Implementation Suggestions**: If you have specific implementation suggestions, you can also propose them here.

### 3. Participate in Code Development

If you are interested in Wutong's code and wish to directly participate in development, you can follow these steps:

> [!IMPORTANT]
> You may have noticed that Wutong is using `gitflow` as its branch management strategy. Therefore, before starting development, please ensure you understand this strategy and how to use it.

1. **Fork the Repository**: Fork Wutong's repository on `GitHub` to your personal account.
2. **Clone the Repository**: Clone the forked repository to your local development environment.
3. **Create a Branch**: Create a new branch for the feature you want to develop or the issue you want to fix.
4. **Write Code**: Write code on the branch and conduct testing.
5. **Commit Code**: Commit your code to the new branch and push it to `GitHub`.
6. **Create a Pull Request**: Create a `Pull Request` on `GitHub` to merge your branch into the corresponding development branch of Wutong.

When submitting code, please try to follow these guidelines:

- **Code Style**: Before submitting, please use `Clippy` to format your code.
- **Testing**: Write test cases for your new or modified features in the `/src/tests/` directory and ensure all tests pass.
- **Documentation**: Update related documentation, including documentation files in the `/docs/` directory, the `README.md` file in the root directory, etc.

> [!CAUTION]
> Please be sure to follow the above rules; otherwise, your code may be rejected for merging.

## Contact Us

If you have any questions or suggestions, please feel free to contact us via the following methods:

- Gavin Zheng <gav.zheng@outlook.com>
- Github Repository: [WutongDev/wutong](https://github.com/WutongDev/wutong)
