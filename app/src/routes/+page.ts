import {redirect} from '@sveltejs/kit';

/**
 * 根路径重定向到欢迎页
 */
export const load = () => {
  throw redirect(307, '/welcome');
};