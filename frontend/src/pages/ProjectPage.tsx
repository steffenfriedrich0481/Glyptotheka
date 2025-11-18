import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { projectsAPI } from '../api/projects';
import { downloadAPI } from '../api/download';
import { downloadUtils } from '../utils/download';
import { ProjectWithRelations, Tag } from '../types/project';
import LoadingSpinner from '../components/common/LoadingSpinner';
import FileList from '../components/project/FileList';
import ImageGallery from '../components/project/ImageGallery';
import ImageCarousel from '../components/project/ImageCarousel';
import { TagManager } from '../components/project/TagManager';

const ProjectPage: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const [project, setProject] = useState<ProjectWithRelations | null>(null);
  const [loading, setLoading] = useState(true);
  const [downloading, setDownloading] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [files, setFiles] = useState<{ stl_files: any[], images: any[] } | null>(null);
  const [images, setImages] = useState<any[]>([]);
  const [imagesPage, setImagesPage] = useState(1);
  const [totalImages, setTotalImages] = useState(0);
  const navigate = useNavigate();

  useEffect(() => {
    if (id) {
      loadProject(parseInt(id));
      loadFiles(parseInt(id));
      loadImages(parseInt(id), 1);
    }
  }, [id]);

  const loadProject = async (projectId: number) => {
    try {
      const data = await projectsAPI.getProject(projectId);
      setProject(data);
    } catch (err) {
      console.error('Failed to load project:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleTagsChange = (tags: Tag[]) => {
    if (project) {
      setProject({ ...project, tags });
    }
  };

  const loadFiles = async (projectId: number) => {
    try {
      const data = await projectsAPI.getProjectFiles(projectId, 1, 1000);
      setFiles(data);
    } catch (err) {
      console.error('Failed to load files:', err);
    }
  };

  const loadImages = async (projectId: number, page: number) => {
    try {
      const data = await projectsAPI.getProjectFiles(projectId, page, 20);
      setImages(data.images || []);
      setTotalImages(data.total_images || 0);
      setImagesPage(page);
    } catch (err) {
      console.error('Failed to load images:', err);
    }
  };

  const handleDownloadAll = async () => {
    if (!project) return;
    
    try {
      setDownloading(true);
      setDownloadProgress(0);
      
      const blob = await downloadAPI.downloadProjectZip(project.id);
      setDownloadProgress(100);
      
      const zipFilename = `${project.name.replace(/\//g, '_')}.zip`;
      downloadUtils.triggerDownload(blob, zipFilename);
    } catch (error) {
      console.error('Failed to download project:', error);
      alert('Failed to download project files. Please try again.');
    } finally {
      setDownloading(false);
      setDownloadProgress(0);
    }
  };

  if (loading) return <LoadingSpinner />;
  if (!project) return <div>Project not found</div>;

  const hasFiles = (project.stl_count || 0) > 0 || (project.image_count || 0) > 0;

  return (
    <div className="container mx-auto p-8">
      <button
        onClick={() => navigate(-1)}
        className="text-blue-500 hover:text-blue-700 mb-4"
      >
        ← Back
      </button>

      <div className="flex justify-between items-center mb-4">
        <div>
          <h1 className="text-3xl font-bold">{project.name}</h1>
          <p className="text-gray-600">{project.full_path}</p>
        </div>
        {hasFiles && (
          <button
            onClick={handleDownloadAll}
            disabled={downloading}
            className="px-6 py-2 bg-green-500 text-white rounded hover:bg-green-600 disabled:bg-gray-400 disabled:cursor-not-allowed"
          >
            {downloading ? 'Preparing Download...' : 'Download All as ZIP'}
          </button>
        )}
      </div>

      {downloading && downloadProgress > 0 && (
        <div className="mb-4">
          <div className="w-full bg-gray-200 rounded-full h-2.5">
            <div
              className="bg-blue-600 h-2.5 rounded-full transition-all duration-300"
              style={{ width: `${downloadProgress}%` }}
            />
          </div>
          <p className="text-sm text-gray-600 mt-1">{downloadProgress}% complete</p>
        </div>
      )}

      {/* Image Carousel Preview */}
      {images.length > 0 && (
        <ImageCarousel images={images} />
      )}

      <div className="mb-8">
        <TagManager 
          projectId={project.id} 
          initialTags={project.tags || []}
          onTagsChange={handleTagsChange}
        />
      </div>

      {project.children.length > 0 && (
        <div className="mb-8">
          <h2 className="text-2xl font-bold mb-4">Sub-projects</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {project.children.map((child) => (
              <div
                key={child.id}
                className="bg-white dark:bg-gray-800 shadow-md rounded-lg p-6 cursor-pointer hover:shadow-xl transition-shadow border border-gray-200 dark:border-gray-700"
                onClick={() => navigate(`/project/${child.id}`)}
              >
                <div className="flex items-center gap-3">
                  <div className="text-3xl">📁</div>
                  <div className="flex-1">
                    <h3 className="font-bold text-lg text-gray-900 dark:text-white">{child.name}</h3>
                    {child.is_leaf && (
                      <span className="text-xs text-gray-500 dark:text-gray-400">Contains files</span>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {images.length > 0 && (
        <div className="mb-8">
          <ImageGallery
            images={images}
            total={totalImages}
            page={imagesPage}
            perPage={20}
            onPageChange={(page) => id && loadImages(parseInt(id), page)}
          />
        </div>
      )}

      {files && (files.stl_files.length > 0 || files.images.length > 0) && (
        <div className="mb-8">
          <FileList files={files.stl_files} images={files.images} />
        </div>
      )}

      {!hasFiles && project.children.length === 0 && (
        <div className="text-gray-500">
          <p>No files or sub-projects in this project.</p>
        </div>
      )}
    </div>
  );
};

export default ProjectPage;
