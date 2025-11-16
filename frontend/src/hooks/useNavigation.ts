import { useNavigate, useLocation } from 'react-router-dom';

export const useNavigation = () => {
  const navigate = useNavigate();
  const location = useLocation();

  const goToProject = (id: number) => {
    navigate(`/project/${id}`);
  };

  const goToBrowse = () => {
    navigate('/browse');
  };

  const goToHome = () => {
    navigate('/');
  };

  const goBack = () => {
    navigate(-1);
  };

  return {
    goToProject,
    goToBrowse,
    goToHome,
    goBack,
    currentPath: location.pathname,
  };
};
