import type { Practice } from '@/types/practice';

const backendApiUrl = process.env.BACKEND_API_URL;

export const fetchPractices = async (): Promise<Practice[]> => {
  // const response = await fetch(`${backendApiUrl}/practices`);

  // if (!response.ok) {
  //   throw new Error('Failed to fetch practices');
  // }

  // return response.json();

  return [
    {
      id: 'string',
      title: 'string',
      description: 'string',
    }
  ]
};
