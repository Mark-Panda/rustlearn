#!/bin/bash

# 默认值
IMAGE_NAME="system-test"
IMAGE_TAG="latest"
DOCKERFILE="Dockerfile"
BUILD_ARGS=""

# 显示使用方法
usage() {
    echo "用法: $0 [-n image_name] [-t tag] [-f dockerfile] [-b build_args]"
    echo "选项:"
    echo "  -n: 镜像名称 (默认: ${IMAGE_NAME})"
    echo "  -t: 镜像标签 (默认: ${IMAGE_TAG})"
    echo "  -f: Dockerfile 路径 (默认: ${DOCKERFILE})"
    echo "  -b: 构建参数 (可选)"
    exit 1
}

# 解析命令行参数
while getopts "n:t:f:b:h" opt; do
    case $opt in
        n) IMAGE_NAME="$OPTARG";;
        t) IMAGE_TAG="$OPTARG";;
        f) DOCKERFILE="$OPTARG";;
        b) BUILD_ARGS="$OPTARG";;
        h) usage;;
        ?) usage;;
    esac
done

# 显示构建信息
echo "📦 开始构建 Docker 镜像"
echo "镜像名称: ${IMAGE_NAME}"
echo "镜像标签: ${IMAGE_TAG}"
echo "Dockerfile: ${DOCKERFILE}"
[ ! -z "$BUILD_ARGS" ] && echo "构建参数: ${BUILD_ARGS}"

# 构建 Docker 镜像
echo "🚀 执行构建..."
docker build \
    --no-cache \
    ${BUILD_ARGS} \
    -t ${IMAGE_NAME}:${IMAGE_TAG} \
    -f ${DOCKERFILE} \
    .

# 检查构建结果
if [ $? -eq 0 ]; then
    echo "✅ 镜像构建成功！"
    echo "镜像信息："
    docker images | grep ${IMAGE_NAME}
else
    echo "❌ 镜像构建失败！"
    exit 1
fi 